use crate::{Decimal, DecimalError, ErrorKind, NullDecimal};
fn invalid() -> DecimalError {
    DecimalError::new(ErrorKind::Unmarshal, "invalid JSON decimal")
}
fn trim_json(bytes: &[u8]) -> &[u8] {
    {
        let start = bytes
            .iter()
            .position(|b| !b" \t\r\n".contains(b))
            .unwrap_or(bytes.len());
        let end = bytes
            .iter()
            .rposition(|b| !b" \t\r\n".contains(b))
            .map_or(start, |i| i + 1);
        &bytes[start..end]
    }
}
impl Decimal {
    pub fn marshal_json(&self) -> Vec<u8> {
        if self.is_nil() {
            b"null".to_vec()
        } else {
            format!("\"{}\"", self.string()).into_bytes()
        }
    }
    pub fn unmarshal_json(&mut self, bytes: &[u8]) -> Result<(), DecimalError> {
        // JSON whitespace is only space, tab, CR and LF, unlike text parsing.
        let bytes = trim_json(bytes);
        if bytes == b"null" {
            return Ok(());
        }
        let text = if bytes.first() == Some(&b'"') {
            decode_string(bytes)?
        } else if bytes
            .first()
            .is_some_and(|c| *c == b'-' || c.is_ascii_digit())
        {
            // arbitrary_precision validates JSON number grammar without a float.
            serde_json::from_slice::<serde_json::Number>(bytes).map_err(|_| invalid())?;
            std::str::from_utf8(bytes)
                .map_err(|_| invalid())?
                .to_owned()
        } else {
            return Err(invalid());
        };
        let parsed = Self::new_from_string(&text).map_err(|e| e.with_kind(ErrorKind::Unmarshal))?;
        *self = parsed;
        Ok(())
    }
}
impl NullDecimal {
    pub fn marshal_json(&self) -> Vec<u8> {
        if !self.valid {
            b"null".to_vec()
        } else {
            format!("\"{}\"", self.decimal.string()).into_bytes()
        }
    }
    pub fn unmarshal_json(&mut self, bytes: &[u8]) -> Result<(), DecimalError> {
        if trim_json(bytes) == b"null" {
            *self = Self::default();
            return Ok(());
        }
        self.decimal.unmarshal_json(bytes)?;
        self.valid = true;
        Ok(())
    }
}
// Go encoding/json replaces invalid UTF-8 and unpaired UTF-16 surrogates in
// strings. Preserve that behavior before exact decimal parsing rejects them.
fn decode_string(bytes: &[u8]) -> Result<String, DecimalError> {
    if bytes.len() < 2 || bytes.last() != Some(&b'"') {
        return Err(invalid());
    }
    let body = &bytes[1..bytes.len() - 1];
    let mut out = String::new();
    let mut i = 0;
    while i < body.len() {
        match body[i] {
            0..=31 | b'"' => return Err(invalid()),
            b'\\' => {
                i += 1;
                let escape = *body.get(i).ok_or_else(invalid)?;
                i += 1;
                match escape {
                    b'"' | b'\\' | b'/' => out.push(escape as char),
                    b'b' => out.push('\u{8}'),
                    b'f' => out.push('\u{c}'),
                    b'n' => out.push('\n'),
                    b'r' => out.push('\r'),
                    b't' => out.push('\t'),
                    b'u' => {
                        let a = hex4(body, &mut i)?;
                        if (0xd800..=0xdbff).contains(&a) && body.get(i..i + 2) == Some(b"\\u") {
                            let mut next = i + 2;
                            let b = hex4(body, &mut next)?;
                            if (0xdc00..=0xdfff).contains(&b) {
                                i = next;
                                out.push(
                                    char::from_u32(0x10000 + ((a - 0xd800) << 10) + (b - 0xdc00))
                                        .unwrap(),
                                );
                            } else {
                                out.push('\u{fffd}');
                            }
                        } else {
                            out.push(char::from_u32(a).unwrap_or('\u{fffd}'));
                        }
                    }
                    _ => return Err(invalid()),
                }
            }
            _ => {
                let start = i;
                while i < body.len() && body[i] >= 32 && body[i] != b'"' && body[i] != b'\\' {
                    i += 1;
                }
                out.push_str(&String::from_utf8_lossy(&body[start..i]));
            }
        }
    }
    Ok(out)
}
fn hex4(body: &[u8], i: &mut usize) -> Result<u32, DecimalError> {
    let digits = body.get(*i..*i + 4).ok_or_else(invalid)?;
    let mut value = 0;
    for b in digits {
        value = value * 16 + (*b as char).to_digit(16).ok_or_else(invalid)?;
    }
    *i += 4;
    Ok(value)
}
