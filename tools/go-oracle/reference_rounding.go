// Derived from exc-works/decimal arithmetic_scale_test.go, fixed SHA in PROVENANCE.md.
// Copyright (c) 2026 exc-works. MIT; see ../../LICENSE.
package main

import "math/big"

const (
	RoundDown = iota
	RoundUp
	RoundCeiling
	RoundHalfUp
	RoundHalfDown
	RoundHalfEven
	RoundUnnecessary
)

func roundedRationalReference(r *big.Rat, prec int, mode int) (*big.Int, bool) {
	scale := new(big.Int).Exp(big.NewInt(10), big.NewInt(int64(prec)), nil)
	x := new(big.Rat).Mul(r, new(big.Rat).SetInt(scale))
	lo := new(big.Int).Div(x.Num(), x.Denom())
	if new(big.Rat).SetInt(lo).Cmp(x) == 0 {
		return lo, true
	}
	if mode == RoundUnnecessary {
		return nil, false
	}
	hi := new(big.Int).Add(lo, big.NewInt(1))
	switch mode {
	case RoundDown:
		if x.Sign() < 0 {
			return hi, true
		}
		return lo, true
	case RoundUp:
		if x.Sign() < 0 {
			return lo, true
		}
		return hi, true
	case RoundCeiling:
		return hi, true
	}
	distance := new(big.Rat).Sub(x, new(big.Rat).SetInt(lo))
	cmp := distance.Cmp(big.NewRat(1, 2))
	if cmp < 0 {
		return lo, true
	}
	if cmp > 0 {
		return hi, true
	}
	if mode == RoundHalfEven {
		if lo.Bit(0) == 0 {
			return lo, true
		}
		return hi, true
	}
	if (mode == RoundHalfUp) == (x.Sign() > 0) {
		return hi, true
	}
	return lo, true
}
