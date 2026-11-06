use crate::{
    BigRational, Decimal, DecimalError, ErrorKind, RoundingMode,
    integer::{pow10, precision, trunc_div_rem},
    rounding::rounded_div,
};
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{Signed, ToPrimitive, Zero};

pub fn most_significant_bit(value: &BigInt) -> usize {
    assert!(
        !value.is_negative(),
        "MostSignificantBit of not positive number"
    );
    value.bits().saturating_sub(1) as usize
}

fn shortest(text: &str) -> String {
    let (mantissa, exponent) = text.split_once('e').unwrap_or((text, ""));
    let mantissa = if mantissa.contains('.') {
        mantissa.trim_end_matches('0').trim_end_matches('.')
    } else {
        mantissa
    };
    if exponent.is_empty() {
        mantissa.into()
    } else {
        format!("{mantissa}e{exponent}")
    }
}

impl Decimal {
    pub fn new_from_float32(value: f32) -> Self {
        if !value.is_finite() {
            return Self::must_from_string("NaN");
        }
        Self::must_from_string(&shortest(ryu::Buffer::new().format_finite(value)))
    }
    pub fn new_from_float64(value: f64) -> Self {
        if !value.is_finite() {
            return Self::must_from_string("NaN");
        }
        Self::must_from_string(&shortest(ryu::Buffer::new().format_finite(value)))
    }
    pub fn new_from_big_rat(value: &BigRational) -> Result<Self, DecimalError> {
        if value.is_zero() {
            return Ok(Self::new(0));
        }
        let mut den = value.denom().clone();
        let twos = den.trailing_zeros().unwrap_or(0) as usize;
        den >>= twos;
        let mut powers = Vec::new();
        let mut power = BigInt::from(5);
        while (&den % &power).is_zero() {
            powers.push(power.clone());
            power = &power * &power;
        }
        let mut fives = 0usize;
        for (i, power) in powers.iter().enumerate().rev() {
            if (&den % power).is_zero() {
                den /= power;
                fives += 1usize << i;
            }
        }
        if den != BigInt::from(1) {
            return Err(DecimalError::new(
                ErrorKind::InvalidFormat,
                "nonterminating decimal rational",
            ));
        }
        let p = twos.max(fives);
        let mut c = value.numer().clone();
        match twos.cmp(&fives) {
            std::cmp::Ordering::Greater => {
                c *= crate::integer::pow_integer(BigInt::from(5), twos - fives)
            }
            std::cmp::Ordering::Less => c <<= fives - twos,
            std::cmp::Ordering::Equal => {}
        }
        Ok(Self::from_parts(c, p))
    }
    pub fn new_from_big_rat_with_prec(
        value: &BigRational,
        prec: isize,
        mode: RoundingMode,
    ) -> Result<Self, DecimalError> {
        let p = precision(prec);
        if value.is_zero() {
            return Ok(Self::from_parts(BigInt::zero(), p));
        }
        Ok(Self::from_parts(
            rounded_div(&(value.numer() * pow10(p)), value.denom(), mode),
            p,
        ))
    }
    pub fn big_rat(&self) -> BigRational {
        BigRational::new(self.raw().clone(), pow10(self.scale))
    }
    pub fn bit_len(&self) -> usize {
        self.raw().bits() as usize
    }
    pub fn int64(&self) -> (i64, bool) {
        if !self.is_integer() {
            return (0, false);
        }
        match self.int_part().to_i64() {
            Some(n) => (n, true),
            None => (0, false),
        }
    }
    pub fn uint64(&self) -> (u64, bool) {
        if self.is_negative() || !self.is_integer() {
            return (0, false);
        }
        match self.int_part().to_u64() {
            Some(n) => (n, true),
            None => (0, false),
        }
    }
    pub fn float32(&self) -> (f32, bool) {
        let (bits, exact) = self.float_bits(23, 8);
        (f32::from_bits(bits as u32), exact)
    }
    pub fn float64(&self) -> (f64, bool) {
        let (bits, exact) = self.float_bits(52, 11);
        (f64::from_bits(bits), exact)
    }

    // Locate the binary grid, then round the exact integer ratio directly to
    // that grid. f32 does not pass through f64, even at subnormal boundaries.
    fn float_bits(&self, fraction: u32, exponent_bits: u32) -> (u64, bool) {
        if self.is_zero() {
            return (0, true);
        }
        let sign = if self.is_negative() {
            1u64 << (fraction + exponent_bits)
        } else {
            0
        };
        let bias = (1i32 << (exponent_bits - 1)) - 1;
        let min_normal = 1 - bias;
        let min_subnormal = min_normal - fraction as i32;
        let infinity = ((1u64 << exponent_bits) - 1) << fraction;
        if self.scale as u128 > self.raw().bits() as u128 + (-min_subnormal + 2) as u128 {
            return (sign, false);
        }
        let mut n = self.raw().abs();
        let mut d = pow10(self.scale);
        let diff = n.bits() as i128 - d.bits() as i128;
        if diff > bias as i128 + 1 {
            return (sign | infinity, false);
        }
        if diff < min_subnormal as i128 - 1 {
            return (sign, false);
        }
        let mut e = diff as i32;
        let less = if e >= 0 {
            n < (&d << e as usize)
        } else {
            (&n << (-e) as usize) < d
        };
        if less {
            e -= 1;
        }
        if e > bias {
            return (sign | infinity, false);
        }
        let unit = (e - fraction as i32).max(min_subnormal);
        if unit < 0 {
            n <<= (-unit) as usize;
        } else {
            d <<= unit as usize;
        }
        let (mut q, r) = trunc_div_rem(&n, &d);
        let exact = r.is_zero();
        let cmp = (&r << 1usize).cmp(&d);
        if cmp.is_gt() || (cmp.is_eq() && q.is_odd()) {
            q += 1;
        }
        let mut significand = q.to_u64().expect("binary significand fits target width");
        if e < min_normal {
            return (sign | significand, exact);
        }
        if significand == (1u64 << (fraction + 1)) {
            significand >>= 1;
            e += 1;
        }
        if e > bias {
            return (sign | infinity, false);
        }
        let bits = sign | (((e + bias) as u64) << fraction) | (significand - (1u64 << fraction));
        (bits, exact)
    }
}
