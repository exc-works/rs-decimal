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
	if req.Operation != "add" {
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
	got, sg, err := rational(req.Candidate)
	if err != nil {
		return "mismatch", err.Error()
	}
	if !req.Candidate.Initialized || sg != max(sa, sb) || got.Cmp(new(big.Rat).Add(a, b)) != 0 {
		return "mismatch", "coefficient, scale, or initialization differs"
	}
	return "verified", "exact rational sum and target scale agree"
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
