use crate::{Decimal, RoundingMode};

/// Options for one Go-compatible formatting verb (not a printf format parser).
/// Numeric flags apply to d/f/F/e/E/g/G/b. Unknown verbs return Go's error marker.
#[derive(Clone, Copy, Debug)]
pub struct GoFormat {
    /// Go verb, such as `f`, `e`, `g`, `s`, or `v`; the default is `v`.
    pub verb: char,
    /// Minimum output width, including sign and decimal point.
    pub width: Option<usize>,
    /// Verb-specific precision: fractional digits for `f/e`, significant digits for `g`.
    pub precision: Option<usize>,
    /// Pads on the right instead of the left.
    pub left: bool,
    /// Shows a plus sign for nonnegative numeric output.
    pub plus: bool,
    /// Uses a leading space for nonnegative numeric output when `plus` is false.
    pub space: bool,
    /// Uses zero padding for numeric output when left alignment is disabled.
    pub zero: bool,
    /// Retains a decimal point and, for general format, trailing zeros.
    pub alternate: bool,
}
impl Default for GoFormat {
    fn default() -> Self {
        Self {
            verb: 'v',
            width: None,
            precision: None,
            left: false,
            plus: false,
            space: false,
            zero: false,
            alternate: false,
        }
    }
}
impl Decimal {
    /// Canonical fixed-point text, removing fractional trailing zeros.
    pub fn string(&self) -> String {
        let mut out = Vec::new();
        self.append_text(&mut out);
        String::from_utf8(out).unwrap()
    }
    /// Formats fixed-point text retaining every stored decimal place.
    pub fn string_with_trailing_zeros(&self) -> String {
        let mut out = Vec::new();
        self.append_text_with_trailing_zeros(&mut out);
        String::from_utf8(out).unwrap()
    }
    /// Appends ASCII text to the existing buffer. The Go method's error is always nil.
    pub fn append_text(&self, dst: &mut Vec<u8>) {
        self.append_decimal(dst, true)
    }
    /// Appends fixed-point ASCII, retaining scale, without clearing the destination.
    pub fn append_text_with_trailing_zeros(&self, dst: &mut Vec<u8>) {
        self.append_decimal(dst, false)
    }
    fn append_decimal(&self, dst: &mut Vec<u8>, strip: bool) {
        if self.is_zero() {
            dst.push(b'0');
            if !strip && self.scale > 0 {
                dst.push(b'.');
                dst.extend(std::iter::repeat_n(b'0', self.scale));
            }
            return;
        }
        let mut digits = self.raw().magnitude().to_string();
        let mut scale = self.scale;
        if strip {
            let remove = (digits.len() - digits.trim_end_matches('0').len()).min(scale);
            digits.truncate(digits.len() - remove);
            scale -= remove;
        }
        if self.is_negative() {
            dst.push(b'-');
        }
        if scale == 0 {
            dst.extend_from_slice(digits.as_bytes());
        } else if scale >= digits.len() {
            dst.extend_from_slice(b"0.");
            dst.extend(std::iter::repeat_n(b'0', scale - digits.len()));
            dst.extend_from_slice(digits.as_bytes());
        } else {
            let cut = digits.len() - scale;
            dst.extend_from_slice(&digits.as_bytes()[..cut]);
            dst.push(b'.');
            dst.extend_from_slice(&digits.as_bytes()[cut..]);
        }
    }
    /// Formats with groups of three integer digits and retained fractional zeros.
    /// A NUL thousands separator disables grouping; a NUL decimal separator selects a dot.
    pub fn format_with_separators(&self, thousands: char, decimal: char) -> String {
        let base = self.string_with_trailing_zeros();
        let negative = base.starts_with('-');
        let base = base.trim_start_matches('-');
        let (integer, fraction) = base
            .split_once('.')
            .map_or((base, None), |(a, b)| (a, Some(b)));
        let mut out = String::new();
        if negative {
            out.push('-');
        }
        for (i, c) in integer.chars().enumerate() {
            if thousands != '\0' && i > 0 && (integer.len() - i) % 3 == 0 {
                out.push(thousands);
            }
            out.push(c);
        }
        if let Some(frac) = fraction {
            out.push(if decimal == '\0' { '.' } else { decimal });
            out.push_str(frac);
        }
        out
    }
    /// Applies one Go formatting verb and its flags. Unsupported verbs return a `%!` marker.
    ///
    /// # Panics
    /// Panics if fixed-point precision exceeds the native signed integer range.
    /// Output allocation grows with the requested width and precision.
    pub fn format_go(&self, f: GoFormat) -> String {
        let error = || format!("%!{}(decimal.Decimal={})", f.verb, self.string());
        let mut core = match f.verb {
            'v' | 's' => self.string(),
            'q' => format!("\"{}\"", self.string()),
            'd' => {
                if !self.is_integer() {
                    return error();
                }
                let integer = self.int_part();
                let mut digits = integer.magnitude().to_string();
                if let Some(p) = f.precision {
                    if p == 0 && self.is_zero() {
                        digits.clear();
                    } else if digits.len() < p {
                        digits = "0".repeat(p - digits.len()) + &digits;
                    }
                }
                if self.is_negative() {
                    digits.insert(0, '-');
                }
                digits
            }
            'f' | 'F' => {
                let mut s = if let Some(p) = f.precision {
                    self.rescale(
                        isize::try_from(p).expect("precision exceeds native range"),
                        RoundingMode::HalfEven,
                    )
                    .string_with_trailing_zeros()
                } else {
                    self.string_with_trailing_zeros()
                };
                if f.alternate && !s.contains('.') {
                    s.push('.');
                }
                s
            }
            'e' | 'E' => {
                let p = f.precision.unwrap_or(6);
                let (digits, exp) = self.significand();
                let (digits, exp) = round_significand(digits, exp, p.saturating_add(1));
                scientific(
                    &digits,
                    exp,
                    p,
                    f.verb == 'E',
                    self.is_negative(),
                    f.alternate,
                )
            }
            'g' | 'G' => self.general(f.precision, f.verb == 'G', f.alternate),
            'b' => format!("{}p-{}", self.raw().to_str_radix(2), self.scale),
            _ => return error(),
        };
        let numeric = matches!(f.verb, 'd' | 'f' | 'F' | 'e' | 'E' | 'g' | 'G' | 'b');
        if numeric && !core.is_empty() && !core.starts_with('-') {
            if f.plus {
                core.insert(0, '+');
            } else if f.space {
                core.insert(0, ' ');
            }
        }
        let pad = f.width.unwrap_or(0).saturating_sub(core.len());
        if f.left {
            core.push_str(&" ".repeat(pad));
        } else if f.zero && numeric && !(f.verb == 'd' && f.precision.is_some()) {
            let sign = usize::from(core.starts_with(['-', '+', ' ']));
            core.insert_str(sign, &"0".repeat(pad));
        } else {
            core.insert_str(0, &" ".repeat(pad));
        }
        core
    }
    fn significand(&self) -> (String, i128) {
        if self.is_zero() {
            return ("0".into(), 0);
        }
        let digits = self.raw().magnitude().to_string();
        let exp = digits.len() as i128 - 1 - self.scale as i128;
        (digits.trim_end_matches('0').to_owned(), exp)
    }
    fn general(&self, precision: Option<usize>, upper: bool, hash: bool) -> String {
        let n = precision.unwrap_or(6).max(1);
        if self.is_zero() {
            return if hash {
                "0.".to_owned() + &"0".repeat(n - 1)
            } else {
                "0".into()
            };
        }
        let (mut digits, mut exp) = self.significand();
        if precision.is_none() && !hash {
            if !(-4..6).contains(&exp) {
                return strip_scientific(scientific(
                    &digits,
                    exp,
                    digits.len() - 1,
                    upper,
                    self.is_negative(),
                    false,
                ));
            }
            return self.string();
        }
        (digits, exp) = round_significand(digits, exp, n);
        if exp < -4 || exp >= n as i128 {
            let s = scientific(&digits, exp, n - 1, upper, self.is_negative(), hash);
            return if hash { s } else { strip_scientific(s) };
        }
        if hash {
            digits.push_str(&"0".repeat(n - digits.len()));
        } else {
            digits = digits.trim_end_matches('0').to_owned();
        }
        let point = exp + 1;
        let mut core = if point <= 0 {
            "0.".to_owned() + &"0".repeat((-point) as usize) + &digits
        } else if point < (digits.len() as i128) {
            let i = point as usize;
            format!("{}.{}", &digits[..i], &digits[i..])
        } else {
            digits.clone()
                + &"0".repeat(point as usize - digits.len())
                + if hash { "." } else { "" }
        };
        if self.is_negative() {
            core.insert(0, '-');
        }
        core
    }
}
fn round_significand(digits: String, mut exp: i128, keep: usize) -> (String, i128) {
    if digits.len() <= keep {
        return (digits, exp);
    }
    let tail = &digits.as_bytes()[keep..];
    let mut head = digits.as_bytes()[..keep].to_vec();
    let up = tail[0] > b'5'
        || tail[0] == b'5'
            && (tail[1..].iter().any(|c| *c != b'0') || (head[keep - 1] - b'0') % 2 == 1);
    if up {
        let mut carry = true;
        for digit in head.iter_mut().rev() {
            if *digit < b'9' {
                *digit += 1;
                carry = false;
                break;
            }
            *digit = b'0';
        }
        if carry {
            head[0] = b'1';
            exp += 1;
        }
    }
    (String::from_utf8(head).unwrap(), exp)
}
fn scientific(
    digits: &str,
    exp: i128,
    p: usize,
    upper: bool,
    negative: bool,
    hash: bool,
) -> String {
    let mut s = String::new();
    if negative {
        s.push('-');
    }
    s.push(digits.as_bytes()[0] as char);
    if p > 0 {
        s.push('.');
        s.push_str(&digits[1..]);
        s.push_str(&"0".repeat(p - (digits.len() - 1)));
    } else if hash {
        s.push('.');
    }
    s.push(if upper { 'E' } else { 'e' });
    s.push(if exp < 0 { '-' } else { '+' });
    s.push_str(&format!("{:02}", exp.abs()));
    s
}
fn strip_scientific(s: String) -> String {
    let i = s.find(['e', 'E']).unwrap();
    let mantissa = &s[..i];
    if mantissa.contains('.') {
        mantissa
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_owned()
            + &s[i..]
    } else {
        s
    }
}
