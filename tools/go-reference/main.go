package main

import (
	"bufio"
	"bytes"
	"encoding/json"
	"errors"
	"fmt"
	d "github.com/exc-works/decimal"
	"io"
	"math/big"
	"os"
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
	Version int      `json:"schema_version"`
	ID      string   `json:"case_id"`
	Outcome string   `json:"outcome"`
	Result  any      `json:"result,omitempty"`
	Inputs  []state  `json:"inputs_after"`
	Errors  []string `json:"error_kinds"`
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
	switch req.Operation {
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
