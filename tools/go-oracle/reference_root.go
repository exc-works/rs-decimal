// Adapted from fixed upstream independent_oracle_test.go; see PROVENANCE.md.
// Copyright (c) 2026 exc-works. MIT; see ../../LICENSE.
package main

import "math/big"

// Test the exact integer boundaries of the candidate's HalfEven rounding cell.
// No production integer-root or midpoint-rounding implementation is called.
func certifyRoot(x *big.Rat, degree int64, prec int, got state) (string, string) {
	if degree <= 0 || prec < 0 || x.Sign() < 0 && degree%2 == 0 {
		return "uncertified", "invalid root case"
	}
	_, scale, err := rational(got)
	if err != nil {
		return "mismatch", err.Error()
	}
	c, _ := new(big.Int).SetString(got.Coefficient, 10)
	if scale != prec || !got.Initialized || c.Sign() != 0 && c.Sign() != x.Sign() {
		return "mismatch", "root state or sign differs"
	}
	if x.Sign() == 0 {
		if c.Sign() != 0 {
			return "mismatch", "nonzero root of zero"
		}
		return "verified", "exact zero"
	}
	if degree > 131072 || degree*int64(prec) > 524288 {
		return "uncertified", "root oracle budget"
	}
	n := big.NewInt(degree)
	q := new(big.Int).Abs(c)
	target := new(big.Int).Abs(x.Num())
	target.Mul(target, new(big.Int).Exp(big.NewInt(10), big.NewInt(degree*int64(prec)), nil))
	target.Lsh(target, uint(degree))
	upper := new(big.Int).Add(new(big.Int).Lsh(new(big.Int).Set(q), 1), big.NewInt(1))
	upper.Exp(upper, n, nil).Mul(upper, x.Denom())
	if cmp := upper.Cmp(target); cmp < 0 || cmp == 0 && q.Bit(0) != 0 {
		return "mismatch", "above exact HalfEven cell"
	}
	if q.Sign() != 0 {
		lower := new(big.Int).Sub(new(big.Int).Lsh(new(big.Int).Set(q), 1), big.NewInt(1))
		lower.Exp(lower, n, nil).Mul(lower, x.Denom())
		if cmp := lower.Cmp(target); cmp > 0 || cmp == 0 && q.Bit(0) != 0 {
			return "mismatch", "below exact HalfEven cell"
		}
	}
	return "verified", "exact root midpoint cell and tie parity agree"
}
