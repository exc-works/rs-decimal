// Adapted from fixed upstream tests; see PROVENANCE.md. MIT.
package main

import (
	"math/big"
)

// These test-only intervals use exact rational arithmetic. They deliberately
// do not call the production fixed-point, interval, series, or reduction code.
type rationalInterval struct{ lo, hi *big.Rat }

func oracleInteger(n int64) *big.Rat { return new(big.Rat).SetInt64(n) }

// For 1<=x<=2, z=(x-1)/(x+1) is in [0,1/3]. Integrating the geometric
// series gives ln(x)=2*sum(z^(2j+1)/(2j+1)). After n terms, the positive
// tail is at most 2*z^(2n+1)/((2n+1)*(1-z²)). Every operation here is exact.
func oracleLogMantissa(x *big.Rat, n int) rationalInterval {
	z := new(big.Rat).Quo(new(big.Rat).Sub(x, oracleInteger(1)), new(big.Rat).Add(x, oracleInteger(1)))
	z2 := new(big.Rat).Mul(z, z)
	power := new(big.Rat).Set(z)
	sum := new(big.Rat)
	for j := range n {
		term := new(big.Rat).Quo(power, oracleInteger(int64(2*j+1)))
		sum.Add(sum, term)
		power.Mul(power, z2)
	}
	lo := new(big.Rat).Mul(sum, oracleInteger(2))
	den := new(big.Rat).Mul(oracleInteger(int64(2*n+1)), new(big.Rat).Sub(oracleInteger(1), z2))
	tail := new(big.Rat).Quo(new(big.Rat).Mul(power, oracleInteger(2)), den)
	return rationalInterval{lo, new(big.Rat).Add(lo, tail)}
}

func oracleLog(x *big.Rat, n int) rationalInterval {
	m := new(big.Rat).Set(x)
	k := int64(0)
	for m.Cmp(oracleInteger(1)) < 0 {
		m.Mul(m, oracleInteger(2))
		k--
	}
	for m.Cmp(oracleInteger(2)) > 0 {
		m.Quo(m, oracleInteger(2))
		k++
	}
	v := oracleLogMantissa(m, n)
	if k == 0 {
		return v
	}
	l2 := oracleLogMantissa(oracleInteger(2), n)
	lo, hi := l2.lo, l2.hi
	if k < 0 {
		lo, hi = hi, lo
	}
	return rationalInterval{
		new(big.Rat).Add(v.lo, new(big.Rat).Mul(oracleInteger(k), lo)),
		new(big.Rat).Add(v.hi, new(big.Rat).Mul(oracleInteger(k), hi)),
	}
}

// The log bases have strictly positive intervals. All four corner ratios
// therefore enclose division even when the numerator straddles zero.
func oracleRatio(a, b rationalInterval) rationalInterval {
	var lo, hi *big.Rat
	for _, x := range []*big.Rat{a.lo, a.hi} {
		for _, y := range []*big.Rat{b.lo, b.hi} {
			q := new(big.Rat).Quo(x, y)
			if lo == nil || q.Cmp(lo) < 0 {
				lo = q
			}
			if hi == nil || q.Cmp(hi) > 0 {
				hi = q
			}
		}
	}
	return rationalInterval{lo, hi}
}

func oracleExp(x *big.Rat, n, prec int) rationalInterval {
	if x.Sign() < 0 {
		// e>2, so x<=-4*(p+1) implies exp(x)<2^(-4*(p+1)). This
		// independent, deliberately conservative bound certifies tiny results
		// without constructing the enormous positive exponential.
		bound := int64(4 * (prec + 1))
		if x.Cmp(oracleInteger(-bound)) <= 0 {
			den := new(big.Int).Lsh(big.NewInt(1), uint(bound))
			return rationalInterval{new(big.Rat), new(big.Rat).SetFrac(big.NewInt(1), den)}
		}
	}
	r := new(big.Rat).Abs(x)
	squares := 0
	for r.Cmp(big.NewRat(1, 4)) > 0 {
		r.Quo(r, oracleInteger(2))
		squares++
	}
	term, sum := oracleInteger(1), oracleInteger(1)
	for j := 1; j <= n; j++ {
		term.Mul(term, r)
		term.Quo(term, oracleInteger(int64(j)))
		sum.Add(sum, term)
	}
	// Successive omitted terms have ratio at most r/(n+2). Bound their
	// positive sum by a geometric tail beginning at term n+1.
	next := new(big.Rat).Quo(new(big.Rat).Mul(term, r), oracleInteger(int64(n+1)))
	ratio := new(big.Rat).Quo(r, oracleInteger(int64(n+2)))
	tail := new(big.Rat).Quo(next, new(big.Rat).Sub(oracleInteger(1), ratio))
	v := rationalInterval{new(big.Rat).Set(sum), new(big.Rat).Add(sum, tail)}
	for range squares {
		v.lo.Mul(v.lo, v.lo)
		v.hi.Mul(v.hi, v.hi)
	}
	if x.Sign() < 0 {
		return rationalInterval{new(big.Rat).Inv(v.hi), new(big.Rat).Inv(v.lo)}
	}
	return v
}

func independentlyRoundedTranscendental(operation string, x *big.Rat, prec int) *big.Int {
	for _, terms := range []int{32, 64, 128, 256} {
		var v rationalInterval
		switch operation {
		case "ln":
			v = oracleLog(x, terms)
		case "log2":
			v = oracleRatio(oracleLog(x, terms), oracleLog(oracleInteger(2), terms))
		case "log10":
			v = oracleRatio(oracleLog(x, terms), oracleLog(oracleInteger(10), terms))
		case "exp":
			v = oracleExp(x, terms, prec)
		default:
			return nil
		}
		lo, _ := roundedRationalReference(v.lo, prec, RoundHalfEven)
		hi, _ := roundedRationalReference(v.hi, prec, RoundHalfEven)
		if lo.Cmp(hi) == 0 {
			return lo
		}
	}
	return nil
}
