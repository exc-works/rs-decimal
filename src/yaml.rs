use crate::{BigInt, Decimal, DecimalError, ErrorKind, NullDecimal};
use num_traits::ToPrimitive;
use yaml_rust2::{
    parser::{Event, Parser, Tag},
    scanner::TScalarStyle,
};
fn invalid() -> DecimalError {
    DecimalError::new(ErrorKind::Unmarshal, "expected one numeric YAML scalar")
}
fn number_error() -> DecimalError {
    DecimalError::new(ErrorKind::InvalidFormat, "invalid YAML number")
        .with_kind(ErrorKind::Unmarshal)
}
impl Decimal {
    /// Semantic YAML string scalar; pass it to the YAML emitter of your choice.
    pub fn marshal_yaml(&self) -> String {
        self.string()
    }
    /// Parses a scalar document from its source, preserving quoting, tags and digits.
    pub fn unmarshal_yaml(&mut self, source: &str) -> Result<(), DecimalError> {
        if let Some(value) = decode(source)? {
            *self = value;
        }
        Ok(())
    }
}
impl NullDecimal {
    /// Returns a semantic string scalar if valid, otherwise a null scalar (`None`).
    pub fn marshal_yaml(&self) -> Option<String> {
        self.valid.then(|| self.decimal.string())
    }
    /// Decodes a scalar document atomically; null/empty documents clear the nullable value.
    pub fn unmarshal_yaml(&mut self, source: &str) -> Result<(), DecimalError> {
        *self = decode(source)?.map(Self::new).unwrap_or_default();
        Ok(())
    }
}
fn decode(source: &str) -> Result<Option<Decimal>, DecimalError> {
    let mut parser = Parser::new_from_str(source);
    let mut docs = 0;
    let mut scalar = None;
    loop {
        match parser.next_token().map_err(|_| invalid())?.0 {
            Event::StreamStart => {}
            Event::DocumentEnd => {}
            Event::DocumentStart => {
                docs += 1;
            }
            Event::StreamEnd => break,
            _ if docs > 1 => {}
            Event::Scalar(value, style, _, tag) => {
                if scalar.is_some() {
                    return Err(invalid());
                }
                scalar = Some((value, style, tag));
            }
            _ => return Err(invalid()),
        }
    }
    let Some((text, style, tag)) = scalar else {
        return Ok(None);
    };
    scalar_value(&text, style, tag.as_ref())
}
fn scalar_value(
    text: &str,
    style: TScalarStyle,
    tag: Option<&Tag>,
) -> Result<Option<Decimal>, DecimalError> {
    if matches!(style, TScalarStyle::Literal | TScalarStyle::Folded) {
        return Err(number_error());
    }
    let tagged = tag.map(|t| {
        if t.handle == "tag:yaml.org,2002:" {
            t.suffix.as_str()
        } else {
            "custom"
        }
    });
    let plain = style == TScalarStyle::Plain;
    if tagged == Some("null")
        || tagged.is_none() && plain && matches!(text, "" | "null" | "Null" | "NULL" | "~")
    {
        return Ok(None);
    }
    if matches!(tagged, Some("bool" | "timestamp"))
        || tagged.is_none()
            && plain
            && matches!(text, "true" | "True" | "TRUE" | "false" | "False" | "FALSE")
    {
        return Err(invalid());
    }
    let clean = text.replace('_', "");
    let integer = base_zero_integer(&clean);
    let bounded = integer
        .as_ref()
        .is_some_and(|i| i.to_i64().is_some() || i.to_u64().is_some());
    let explicit_int = tagged == Some("int");
    let explicit_float = tagged == Some("float");
    let numeric_plain = tagged.is_none() && plain && !text.starts_with('_');
    let implicit_int = numeric_plain && bounded && {
        let magnitude = clean.trim_start_matches(['-', '+']);
        // The baseline token resolver recognizes only lower-case radix prefixes.
        !["0X", "0O", "0B"]
            .iter()
            .any(|prefix| magnitude.starts_with(prefix))
    };
    let implicit_float = numeric_plain
        && text.matches('.').count() == 1
        && clean.parse::<f64>().is_ok_and(|f| f.is_finite());
    if explicit_int || implicit_int || explicit_float && bounded {
        return integer
            .map(|i| Some(Decimal::new_from_big_int(&i)))
            .ok_or_else(number_error);
    }
    let normalized = if explicit_float || implicit_float {
        normalize_float(&clean)
    } else {
        text.to_owned()
    };
    Decimal::new_from_string(&normalized)
        .map(Some)
        .map_err(|e| e.with_kind(ErrorKind::Unmarshal))
}
fn base_zero_integer(text: &str) -> Option<BigInt> {
    let (negative, digits) = if let Some(s) = text.strip_prefix('-') {
        (true, s)
    } else {
        (false, text.strip_prefix('+').unwrap_or(text))
    };
    let (radix, digits) = if let Some(s) = digits
        .strip_prefix("0x")
        .or_else(|| digits.strip_prefix("0X"))
    {
        (16, s)
    } else if let Some(s) = digits
        .strip_prefix("0o")
        .or_else(|| digits.strip_prefix("0O"))
    {
        (8, s)
    } else if let Some(s) = digits
        .strip_prefix("0b")
        .or_else(|| digits.strip_prefix("0B"))
    {
        (2, s)
    } else if digits.starts_with('0') && digits.len() > 1 {
        (8, &digits[1..])
    } else {
        (10, digits)
    };
    if digits.is_empty() || !digits.chars().all(|c| c.is_digit(radix)) {
        return None;
    }
    let n = BigInt::parse_bytes(digits.as_bytes(), radix)?;
    Some(if negative { -n } else { n })
}
fn normalize_float(text: &str) -> String {
    let (mantissa, exponent) = text
        .find(['e', 'E'])
        .map_or((text, ""), |i| (&text[..i], &text[i..]));
    let (sign, mantissa) = if mantissa.starts_with(['-', '+']) {
        (&mantissa[..1], &mantissa[1..])
    } else {
        ("", mantissa)
    };
    format!(
        "{sign}{}{mantissa}{}{exponent}",
        if mantissa.starts_with('.') { "0" } else { "" },
        if mantissa.ends_with('.') { "0" } else { "" }
    )
}
