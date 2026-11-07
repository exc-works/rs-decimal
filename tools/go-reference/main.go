package main

import (
	"bufio"
	"bytes"
	"encoding/hex"
	"encoding/json"
	"encoding/xml"
	"errors"
	"fmt"
	d "github.com/exc-works/decimal"
	"github.com/go-playground/validator/v10"
	yaml "github.com/goccy/go-yaml"
	"io"
	"math"
	"math/big"
	"os"
	"reflect"
	"strconv"
)

type state struct {
	Coefficient string `json:"coefficient"`
	Scale       string `json:"scale"`
	Initialized bool   `json:"initialized"`
}
type request struct {
	Version   int             `json:"schema_version"`
	ID        string          `json:"case_id"`
	Operation string          `json:"operation"`
	Inputs    []state         `json:"inputs"`
	Params    json.RawMessage `json:"params"`
}
type response struct {
	Receiver any      `json:"receiver_after,omitempty"`
	Version  int      `json:"schema_version"`
	ID       string   `json:"case_id"`
	Outcome  string   `json:"outcome"`
	Result   any      `json:"result,omitempty"`
	Inputs   []state  `json:"inputs_after"`
	Errors   []string `json:"error_kinds"`
}

func capture(v d.Decimal) state {
	return state{v.BigInt().String(), strconv.Itoa(v.Precision()), !v.IsNil()}
}
func decimal(s state) (d.Decimal, error) {
	c, ok := new(big.Int).SetString(s.Coefficient, 10)
	if !ok || c.String() != s.Coefficient {
		return d.Decimal{}, fmt.Errorf("invalid coefficient")
	}
	p, err := strconv.Atoi(s.Scale)
	if err != nil || p < 0 || strconv.Itoa(p) != s.Scale {
		return d.Decimal{}, fmt.Errorf("invalid scale")
	}
	if !s.Initialized {
		if c.Sign() != 0 || p != 0 {
			return d.Decimal{}, fmt.Errorf("invalid nil state")
		}
		return d.Decimal{}, nil
	}
	return d.NewFromBigIntWithPrec(c, p), nil
}
func kinds(err error) []string {
	out := []string{}
	for _, e := range []struct {
		name string
		err  error
	}{
		{"DivideByZero", d.ErrDivideByZero}, {"InvalidArgument", d.ErrInvalidArgument}, {"InvalidFormat", d.ErrInvalidFormat}, {"InvalidLog", d.ErrInvalidLog}, {"InvalidPrecision", d.ErrInvalidPrecision}, {"InvalidRoot", d.ErrInvalidRoot}, {"NegativeRoot", d.ErrNegativeRoot}, {"NoConvergence", d.ErrNoConvergence}, {"Overflow", d.ErrOverflow}, {"RoundUnnecessary", d.ErrRoundUnnecessary}, {"Unmarshal", d.ErrUnmarshal},
	} {
		if errors.Is(err, e.err) {
			out = append(out, e.name)
		}
	}
	if len(out) == 0 {
		out = append(out, "Unclassified")
	}
	return out
}
func execute(req request) (r response) {
	r = response{Version: 1, ID: req.ID, Outcome: "unsupported", Inputs: []state{}, Errors: []string{}}
	values := []d.Decimal{}
	defer func() {
		for _, v := range values {
			r.Inputs = append(r.Inputs, capture(v))
		}
		if p := recover(); p != nil {
			r.Outcome = "panic"
			r.Result = nil
			if err, ok := p.(error); ok {
				r.Errors = kinds(err)
			} else {
				r.Errors = []string{"Assertion"}
			}
		}
	}()
	if req.Version != 1 || req.ID == "" {
		r.Outcome = "protocol_error"
		return
	}
	for _, s := range req.Inputs {
		v, err := decimal(s)
		if err != nil {
			r.Outcome = "protocol_error"
			return
		}
		values = append(values, v)
	}
	params := map[string]string{}
	if len(req.Params) > 0 {
		if err := json.Unmarshal(req.Params, &params); err != nil {
			r.Outcome = "protocol_error"
			return
		}
	}
	set := func(v d.Decimal, err error) {
		if err != nil {
			r.Outcome = "error"
			r.Errors = kinds(err)
		} else {
			r.Outcome = "ok"
			r.Result = capture(v)
		}
	}
	integer := func(key string) int {
		s := params[key]
		if s == "" {
			return 0
		}
		n, err := strconv.Atoi(s)
		if err != nil {
			panic(err)
		}
		return n
	}
	switch req.Operation {
	case "decode":
		nullable := params["nullable"] == "true"
		n := d.NullDecimal{Decimal: values[0], Valid: params["valid"] == "true"}
		var err error
		switch {
		case params["format"] == "sql":
			var value any
			switch params["type"] {
			case "null":
				value = nil
			case "i64":
				value, err = strconv.ParseInt(params["text"], 10, 64)
			case "f32":
				var bits uint64
				bits, err = strconv.ParseUint(params["text"], 16, 32)
				value = math.Float32frombits(uint32(bits))
			case "f64":
				var bits uint64
				bits, err = strconv.ParseUint(params["text"], 16, 64)
				value = math.Float64frombits(bits)
			case "text":
				value = params["text"]
			case "bytes":
				value, err = hex.DecodeString(params["text"])
			default:
				value = true
			}
			if err != nil {
				panic(err)
			}
			if nullable {
				err = n.Scan(value)
			} else {
				err = n.Decimal.Scan(value)
			}
		case params["format"] == "bson":
			data, e := hex.DecodeString(params["text"])
			if e != nil {
				panic(e)
			}
			if nullable {
				err = n.UnmarshalBSONValue(byte(integer("type")), data)
			} else {
				err = n.Decimal.UnmarshalBSONValue(byte(integer("type")), data)
			}
		case !nullable && params["format"] == "binary":
			bytes, e := hex.DecodeString(params["text"])
			if e != nil {
				panic(e)
			}
			err = n.Decimal.UnmarshalBinary(bytes)
		case nullable && params["format"] == "xml":
			err = decodeXML(params["text"], &n)
		case !nullable && params["format"] == "xml":
			err = decodeXML(params["text"], &n.Decimal)
		case nullable && params["format"] == "xml_attr":
			err = n.UnmarshalXMLAttr(xml.Attr{Value: params["text"]})
		case !nullable && params["format"] == "xml_attr":
			err = n.Decimal.UnmarshalXMLAttr(xml.Attr{Value: params["text"]})
		case nullable && params["format"] == "yaml":
			err = n.UnmarshalYAML(func(target any) error { return yaml.Unmarshal([]byte(params["text"]), target) })
		case !nullable && params["format"] == "yaml":
			err = n.Decimal.UnmarshalYAML(func(target any) error { return yaml.Unmarshal([]byte(params["text"]), target) })
		case nullable && params["format"] == "json":
			err = n.UnmarshalJSON([]byte(params["text"]))
		case !nullable && params["format"] == "json":
			err = n.Decimal.UnmarshalJSON([]byte(params["text"]))
		case nullable && params["format"] == "text":
			err = n.UnmarshalText([]byte(params["text"]))
		case nullable && params["format"] == "param":
			err = n.UnmarshalParam(params["text"])
		case !nullable && params["format"] == "text":
			err = n.Decimal.UnmarshalText([]byte(params["text"]))
		case !nullable && params["format"] == "param":
			err = n.Decimal.UnmarshalParam(params["text"])
		default:
			panic("unknown decoder")
		}
		var valid any
		if nullable {
			valid = n.Valid
		}
		r.Receiver = map[string]any{"decimal": capture(n.Decimal), "valid": valid}
		if err != nil {
			r.Outcome = "error"
			r.Errors = kinds(err)
		} else {
			r.Outcome = "ok"
			r.Result = r.Receiver
		}
	case "validate":
		v := validator.New()
		if err := d.RegisterGoPlaygroundValidator(v); err != nil {
			panic(err)
		}
		tag := params["tag"]
		if params["parameter"] != "" {
			tag += "=" + params["parameter"]
		}
		r.Outcome = "ok"
		typ := reflect.StructOf([]reflect.StructField{{Name: "Amount", Type: reflect.TypeOf(d.Decimal{}), Tag: reflect.StructTag(`validate:"` + tag + `"`)}})
		sample := reflect.New(typ).Elem()
		sample.Field(0).Set(reflect.ValueOf(values[0]))
		r.Result = v.Struct(sample.Interface()) == nil
	case "messages":
		r.Outcome = "ok"
		r.Result = d.DefaultGoPlaygroundValidatorTranslationMessages(params["locale"])
	case "sql_value":
		n := d.NullDecimal{Decimal: values[0], Valid: params["valid"] == "true"}
		r.Outcome = "ok"
		if params["nullable"] == "true" {
			r.Result, _ = n.Value()
		} else {
			r.Result, _ = n.Decimal.Value()
		}
		if r.Result == nil {
			r.Result = json.RawMessage("null")
		}
	case "encode_bson":
		n := d.NullDecimal{Decimal: values[0], Valid: params["valid"] == "true"}
		var kind byte
		var data []byte
		var err error
		if params["nullable"] == "true" {
			kind, data, err = n.MarshalBSONValue()
		} else {
			kind, data, err = n.Decimal.MarshalBSONValue()
		}
		if err != nil {
			r.Outcome = "error"
			r.Errors = kinds(err)
		} else {
			r.Outcome = "ok"
			r.Result = map[string]string{"type": strconv.Itoa(int(kind)), "data": hex.EncodeToString(data)}
		}
	case "encode_binary":
		bytes, err := values[0].MarshalBinary()
		if err != nil {
			r.Outcome = "error"
			r.Errors = kinds(err)
		} else {
			r.Outcome = "ok"
			r.Result = hex.EncodeToString(bytes)
		}
	case "encode_xml":
		n := d.NullDecimal{Decimal: values[0], Valid: params["valid"] == "true"}
		r.Outcome = "ok"
		if params["format"] == "attribute" {
			var attr xml.Attr
			if params["nullable"] == "true" {
				attr, _ = n.MarshalXMLAttr(xml.Name{Local: "v"})
			} else {
				attr, _ = n.Decimal.MarshalXMLAttr(xml.Name{Local: "v"})
			}
			if attr.Name.Local == "" {
				r.Result = json.RawMessage("null")
			} else {
				r.Result = attr.Value
			}
		} else {
			var out bytes.Buffer
			encoder := xml.NewEncoder(&out)
			start := xml.StartElement{Name: xml.Name{Local: "v"}}
			if params["nullable"] == "true" {
				_ = n.MarshalXML(encoder, start)
			} else {
				_ = n.Decimal.MarshalXML(encoder, start)
			}
			_ = encoder.Flush()
			r.Result = out.String()
		}
	case "encode_yaml":
		n := d.NullDecimal{Decimal: values[0], Valid: params["valid"] == "true"}
		var value any
		if params["nullable"] == "true" {
			value, _ = n.MarshalYAML()
		} else {
			value, _ = n.Decimal.MarshalYAML()
		}
		r.Outcome = "ok"
		if value == nil {
			r.Result = json.RawMessage("null")
		} else {
			r.Result = value
		}
	case "encode_json":
		n := d.NullDecimal{Decimal: values[0], Valid: params["valid"] == "true"}
		var out []byte
		if params["nullable"] == "true" {
			out, _ = n.MarshalJSON()
		} else {
			out, _ = n.Decimal.MarshalJSON()
		}
		r.Outcome = "ok"
		r.Result = string(out)
	case "encode_text":
		n := d.NullDecimal{Decimal: values[0], Valid: params["valid"] == "true"}
		var out []byte
		if params["nullable"] == "true" {
			if params["format"] == "string" {
				out = []byte(n.String())
			} else {
				out, _ = n.MarshalText()
			}
		} else {
			out, _ = n.Decimal.MarshalText()
		}
		r.Outcome = "ok"
		r.Result = string(out)
	case "format":
		format := "%" + params["flags"] + params["width"]
		if p, ok := params["precision"]; ok {
			format += "." + p
		}
		format += params["verb"]
		r.Outcome = "ok"
		r.Result = fmt.Sprintf(format, values[0])
	case "string":
		r.Outcome = "ok"
		r.Result = values[0].String()
	case "trailing":
		r.Outcome = "ok"
		r.Result = values[0].StringWithTrailingZeros()
	case "separators":
		r.Outcome = "ok"
		r.Result = values[0].FormatWithSeparators([]rune(params["thousands"])[0], []rune(params["decimal"])[0])
	case "append":
		var out []byte
		if params["trailing"] == "true" {
			out, _ = values[0].AppendTextWithTrailingZeros([]byte(params["prefix"]))
		} else {
			out, _ = values[0].AppendText([]byte(params["prefix"]))
		}
		r.Outcome = "ok"
		r.Result = string(out)
	case "ln":
		set(values[0].Ln())
	case "ln_prec":
		set(values[0].LnWithPrec(integer("precision")))
	case "log2":
		set(values[0].Log2(), nil)
	case "log2_prec":
		set(values[0].Log2WithPrec(integer("precision")))
	case "log10":
		set(values[0].Log10())
	case "log10_prec":
		set(values[0].Log10WithPrec(integer("precision")))
	case "exp":
		set(values[0].Exp())
	case "exp_prec":
		set(values[0].ExpWithPrec(integer("precision")))
	case "sqrt":
		set(values[0].Sqrt())
	case "sqrt_prec":
		set(values[0].SqrtWithPrec(integer("precision")))
	case "root":
		n, err := strconv.ParseInt(params["degree"], 10, 64)
		if err != nil {
			panic(err)
		}
		set(values[0].ApproxRoot(n))
	case "root_prec":
		n, err := strconv.ParseInt(params["degree"], 10, 64)
		if err != nil {
			panic(err)
		}
		set(values[0].ApproxRootWithPrec(n, integer("precision")))
	case "from_float":
		bits, err := strconv.ParseUint(params["bits"], 16, 64)
		if err != nil {
			panic(err)
		}
		if params["width"] == "32" {
			set(d.NewFromFloat32(math.Float32frombits(uint32(bits))), nil)
		} else {
			set(d.NewFromFloat64(math.Float64frombits(bits)), nil)
		}
	case "to_float":
		r.Outcome = "ok"
		if params["width"] == "32" {
			v, e := values[0].Float32()
			r.Result = map[string]any{"width": "32", "bits": fmt.Sprintf("%08x", math.Float32bits(v)), "exact": e}
		} else {
			v, e := values[0].Float64()
			r.Result = map[string]any{"width": "64", "bits": fmt.Sprintf("%016x", math.Float64bits(v)), "exact": e}
		}
	case "from_rat", "from_rat_prec":
		n, ok := new(big.Int).SetString(params["numerator"], 10)
		if !ok {
			panic("invalid numerator")
		}
		den, ok := new(big.Int).SetString(params["denominator"], 10)
		if !ok {
			panic("invalid denominator")
		}
		v := new(big.Rat).SetFrac(n, den)
		if req.Operation == "from_rat" {
			set(d.NewFromBigRat(v))
		} else {
			set(d.NewFromBigRatWithPrec(v, integer("precision"), d.RoundingMode(integer("mode"))))
		}
	case "to_rat":
		v := values[0].BigRat()
		r.Outcome = "ok"
		r.Result = map[string]any{"numerator": v.Num().String(), "denominator": v.Denom().String()}
	case "native_int":
		a, ea := values[0].Int64()
		b, eb := values[0].Uint64()
		r.Outcome = "ok"
		r.Result = map[string]any{"signed": strconv.FormatInt(a, 10), "signed_exact": ea, "unsigned": strconv.FormatUint(b, 10), "unsigned_exact": eb, "bit_len": strconv.Itoa(values[0].BitLen())}
	case "msb":
		v, ok := new(big.Int).SetString(params["value"], 10)
		if !ok {
			panic("invalid msb input")
		}
		r.Outcome = "ok"
		r.Result = strconv.FormatUint(uint64(d.MostSignificantBit(v)), 10)
	case "add":
		set(values[0].Add(values[1]), nil)
	case "sub":
		set(values[0].Sub(values[1]), nil)
	case "safe_add":
		set(values[0].SafeAdd(values[1]), nil)
	case "safe_sub":
		set(values[0].SafeSub(values[1]), nil)
	case "mul":
		set(values[0].Mul(values[1], d.RoundingMode(integer("mode"))), nil)
	case "mul_down":
		set(values[0].MulDown(values[1]), nil)
	case "mul_exact":
		set(values[0].MulExact(values[1]), nil)
	case "mul_prec":
		set(values[0].MulWithPrec(values[1], integer("precision"), d.RoundingMode(integer("mode"))), nil)
	case "quo":
		set(values[0].Quo(values[1], d.RoundingMode(integer("mode"))), nil)
	case "quo_down":
		set(values[0].QuoDown(values[1]), nil)
	case "quo_prec":
		set(values[0].QuoWithPrec(values[1], integer("precision"), d.RoundingMode(integer("mode"))), nil)
	case "quo_rem":
		q, rem := values[0].QuoRem(values[1])
		r.Outcome = "ok"
		r.Result = []state{capture(q), capture(rem)}
	case "mod":
		set(values[0].Mod(values[1]), nil)
	case "power":
		n, err := strconv.ParseInt(params["value"], 10, 64)
		if err != nil {
			panic(err)
		}
		set(values[0].Power(n), nil)
	case "add_raw":
		n, err := strconv.ParseInt(params["value"], 10, 64)
		if err != nil {
			panic(err)
		}
		set(values[0].AddRaw(n), nil)
	case "sub_raw":
		n, err := strconv.ParseInt(params["value"], 10, 64)
		if err != nil {
			panic(err)
		}
		set(values[0].SubRaw(n), nil)
	case "must_non_negative":
		set(values[0].MustNonNegative(), nil)
	case "integer_parts":
		a := values[0]
		i, f := a.Remainder()
		r.Outcome = "ok"
		r.Result = map[string]any{"int_part": a.IntPart().String(), "integer": i.String(), "fraction": f.String(), "is_integer": a.IsInteger(), "has_fraction": a.HasFraction()}
	case "mode":
		mode := integer("mode")
		d.New(0).Rescale(0, d.RoundingMode(mode))
		r.Outcome = "ok"
		r.Result = strconv.Itoa(mode)
	case "rescale":
		set(values[0].Rescale(integer("precision"), d.RoundingMode(integer("mode"))), nil)
	case "rescale_down":
		set(values[0].RescaleDown(integer("precision")), nil)
	case "floor":
		set(values[0].FloorWithPrec(integer("precision")), nil)
	case "ceil":
		set(values[0].CeilWithPrec(integer("precision")), nil)
	case "round":
		set(values[0].RoundWithPrec(integer("precision")), nil)
	case "truncate":
		set(values[0].TruncateWithPrec(integer("precision")), nil)
	case "shift":
		set(values[0].Shift(integer("places")), nil)
	case "strip":
		set(values[0].StripTrailingZeros(), nil)
	case "significant":
		set(values[0].SignificantFigures(integer("figures"), d.RoundingMode(integer("mode"))), nil)
	case "parse":
		set(d.NewFromString(params["text"]))
	case "must_parse":
		set(d.MustFromString(params["text"]), nil)
	case "construct":
		p := integer("precision")
		i, _ := strconv.ParseInt(params["value"], 10, 64)
		u, _ := strconv.ParseUint(params["value"], 10, 64)
		b, _ := new(big.Int).SetString(params["value"], 10)
		switch params["kind"] {
		case "new":
			set(d.New(i), nil)
		case "int":
			set(d.NewFromInt(int(i)), nil)
		case "with_prec":
			set(d.NewWithPrec(i, p), nil)
		case "int64":
			set(d.NewFromInt64(i, p), nil)
		case "uint64":
			set(d.NewFromUint64(u, p), nil)
		case "big_int":
			set(d.NewFromBigInt(b), nil)
		case "big_int_prec":
			set(d.NewFromBigIntWithPrec(b, p), nil)
		case "append":
			set(d.NewWithAppendPrec(i, p), nil)
		case "uint_append":
			set(d.NewFromUintWithAppendPrec(u, p), nil)
		}
	case "inspect":
		v := values[0]
		r.Outcome = "ok"
		r.Result = map[string]any{"sign": strconv.Itoa(v.Sign()), "nil": v.IsNil(), "zero": v.IsZero(), "not_zero": v.IsNotZero(), "positive": v.IsPositive(), "negative": v.IsNegative(), "precision": strconv.Itoa(v.Precision())}
	case "cmp":
		a, b := values[0], values[1]
		r.Outcome = "ok"
		r.Result = map[string]any{"cmp": strconv.Itoa(a.Cmp(b)), "equal": a.Equal(b), "not_equal": a.NotEqual(b), "gt": a.GT(b), "gte": a.GTE(b), "lt": a.LT(b), "lte": a.LTE(b)}
	case "max":
		set(values[0].Max(values[1]), nil)
	case "min":
		set(values[0].Min(values[1]), nil)
	case "between":
		r.Outcome = "ok"
		r.Result = d.Between(values[0], values[1], values[2])
	case "clone":
		set(d.NewFromDecimal(values[0]), nil)
	case "abs":
		set(values[0].Abs(), nil)
	case "neg":
		set(values[0].Neg(), nil)
	case "state":
		if len(values) != 1 {
			r.Outcome = "protocol_error"
			return
		}
		r.Outcome = "ok"
		r.Result = capture(values[0])
	}
	return
}
func main() {
	reader := bufio.NewReader(os.Stdin)
	enc := json.NewEncoder(os.Stdout)
	for {
		line, err := reader.ReadBytes('\n')
		if err == io.EOF && len(line) == 0 {
			return
		}
		if err != nil && err != io.EOF {
			panic(err)
		}
		var req request
		dec := json.NewDecoder(bytes.NewReader(line))
		dec.DisallowUnknownFields()
		if e := dec.Decode(&req); e != nil {
			panic(e)
		}
		var extra any
		if e := dec.Decode(&extra); e != io.EOF {
			panic("trailing JSON")
		}
		if e := enc.Encode(execute(req)); e != nil {
			panic(e)
		}
	}
}

// Rust exposes a whole-document convenience method rather than Decoder/StartElement.
// Normalize errors before the Go hook is entered to the public decoder error kind.
func decodeXML(source string, value any) error {
	err := xml.Unmarshal([]byte(source), value)
	if err != nil && !errors.Is(err, d.ErrUnmarshal) {
		return fmt.Errorf("%w: %w", d.ErrUnmarshal, err)
	}
	return err
}
