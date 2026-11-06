// Independent exact arithmetic; deliberately does not import the production library.
package main

import (
	"bufio"
	"bytes"
	"encoding/json"
	"fmt"
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
	Version   int               `json:"schema_version"`
	ID        string            `json:"case_id"`
	Operation string            `json:"operation"`
	Inputs    []state           `json:"inputs"`
	Params    map[string]string `json:"params"`
	Candidate state             `json:"candidate"`
}

func rational(s state) (*big.Rat, int, error) {
	c, ok := new(big.Int).SetString(s.Coefficient, 10)
	p, err := strconv.Atoi(s.Scale)
	if !ok || err != nil || p < 0 || p > 131072 || c.String() != s.Coefficient || strconv.Itoa(p) != s.Scale {
		return nil, 0, fmt.Errorf("invalid or unaffordable state")
	}
	if !s.Initialized && (c.Sign() != 0 || p != 0) {
		return nil, 0, fmt.Errorf("invalid nil state")
	}
	return new(big.Rat).SetFrac(c, new(big.Int).Exp(big.NewInt(10), big.NewInt(int64(p)), nil)), p, nil
}
func verify(req request) (status, detail string) {
	if req.Version != 1 || req.ID == "" {
		return "uncertified", "invalid request"
	}
	if req.Operation == "sqrt" || req.Operation == "root" || req.Operation == "sqrt_prec" || req.Operation == "root_prec" {
		if len(req.Inputs) != 1 {
			return "uncertified", "expected one operand"
		}
		x, scale, err := rational(req.Inputs[0])
		if err != nil {
			return "uncertified", err.Error()
		}
		degree := int64(2)
		if req.Operation == "root" || req.Operation == "root_prec" {
			degree, err = strconv.ParseInt(req.Params["degree"], 10, 64)
			if err != nil {
				return "uncertified", "invalid degree"
			}
		}
		p := max(scale, 20)
		if req.Operation == "sqrt_prec" || req.Operation == "root_prec" {
			p, err = strconv.Atoi(req.Params["precision"])
			if err != nil {
				return "uncertified", "invalid precision"
			}
		}
		return certifyRoot(x, degree, p, req.Candidate)
	}
	if req.Operation == "rescale" {
		if len(req.Inputs) != 1 {
			return "uncertified", "expected one operand"
		}
		x, _, err := rational(req.Inputs[0])
		if err != nil {
			return "uncertified", err.Error()
		}
		p, err := strconv.Atoi(req.Params["precision"])
		if err != nil || p < 0 || p > 131072 {
			return "uncertified", "invalid precision"
		}
		mode, err := strconv.Atoi(req.Params["mode"])
		if err != nil || mode < 0 || mode > 6 {
			return "uncertified", "invalid mode"
		}
		_, sg, err := rational(req.Candidate)
		if err != nil {
			return "mismatch", err.Error()
		}
		want, ok := roundedRationalReference(x, p, mode)
		if !ok {
			return "mismatch", "successful rounding forbidden by Unnecessary"
		}
		if !req.Candidate.Initialized || sg != p || req.Candidate.Coefficient != want.String() {
			return "mismatch", "rounded coefficient or state differs"
		}
		return "verified", "independent exact distances to grid neighbors agree"
	}
	if req.Operation == "power" {
		if len(req.Inputs) != 1 {
			return "uncertified", "expected one operand"
		}
		a, p, err := rational(req.Inputs[0])
		if err != nil {
			return "uncertified", err.Error()
		}
		n, err := strconv.ParseInt(req.Params["value"], 10, 64)
		if err != nil || n < -1000 || n > 1000 {
			return "uncertified", "oracle power budget"
		}
		if a.Sign() == 0 && n < 0 {
			return "mismatch", "zero negative power cannot succeed"
		}
		magnitude := n
		if n < 0 {
			magnitude = -n
		}
		num := new(big.Int).Exp(a.Num(), big.NewInt(magnitude), nil)
		den := new(big.Int).Exp(a.Denom(), big.NewInt(magnitude), nil)
		exact := new(big.Rat).SetFrac(num, den)
		if n < 0 {
			exact.Inv(exact)
		}
		return verifyRounded(exact, p, RoundHalfEven, req.Candidate)
	}
	switch req.Operation {
	case "add", "sub", "mul", "mul_exact", "mul_prec", "quo", "quo_prec":
	default:
		return "unsupported", "unknown operation"
	}
	if len(req.Inputs) != 2 {
		return "uncertified", "expected two operands"
	}
	a, sa, err := rational(req.Inputs[0])
	if err != nil {
		return "uncertified", err.Error()
	}
	b, sb, err := rational(req.Inputs[1])
	if err != nil {
		return "uncertified", err.Error()
	}
	p := max(sa, sb)
	mode := RoundUnnecessary
	exact := new(big.Rat)
	switch req.Operation {
	case "add":
		exact.Add(a, b)
	case "sub":
		exact.Sub(a, b)
	case "mul_exact":
		exact.Mul(a, b)
		p = sa + sb
	default:
		mode, err = strconv.Atoi(req.Params["mode"])
		if err != nil || mode < 0 || mode > 6 {
			return "uncertified", "invalid mode"
		}
		if req.Operation == "mul_prec" || req.Operation == "quo_prec" {
			p, err = strconv.Atoi(req.Params["precision"])
			if err != nil {
				return "uncertified", "invalid precision"
			}
		}
		if req.Operation == "mul" || req.Operation == "mul_prec" {
			exact.Mul(a, b)
		} else {
			if b.Sign() == 0 {
				return "mismatch", "division by zero cannot succeed"
			}
			exact.Quo(a, b)
		}
	}
	return verifyRounded(exact, p, mode, req.Candidate)
}
func verifyRounded(x *big.Rat, p, mode int, candidate state) (string, string) {
	if p < 0 || p > 131072 {
		return "uncertified", "oracle precision budget"
	}
	_, sg, err := rational(candidate)
	if err != nil {
		return "mismatch", err.Error()
	}
	want, ok := roundedRationalReference(x, p, mode)
	if !ok {
		return "mismatch", "Unnecessary forbids successful result"
	}
	if !candidate.Initialized || sg != p || candidate.Coefficient != want.String() {
		return "mismatch", "coefficient, scale, or initialization differs"
	}
	return "verified", "exact rational operation and independent grid-neighbor rounding agree"
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
		status, detail := verify(req)
		if err := enc.Encode(map[string]any{"schema_version": 1, "case_id": req.ID, "status": status, "detail": detail}); err != nil {
			panic(err)
		}
	}
}
