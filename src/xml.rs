use crate::{Decimal, DecimalError, ErrorKind, NullDecimal};
use quick_xml::{Reader, events::Event};
fn invalid() -> DecimalError {
    DecimalError::new(ErrorKind::Unmarshal, "invalid XML decimal element")
}
impl Decimal {
    /// Character content for an XML element; the caller's emitter supplies its name/namespace.
    pub fn marshal_xml(&self) -> String {
        if self.is_nil() {
            String::new()
        } else {
            self.string()
        }
    }
    /// Decodes the first XML element, as Go's Decoder.Decode does.
    pub fn unmarshal_xml(&mut self, source: &str) -> Result<(), DecimalError> {
        self.unmarshal_xml_attr(&element_text(source)?)
    }
    /// Decoded attribute value; the caller's XML emitter performs attribute escaping.
    pub fn marshal_xml_attr(&self) -> String {
        self.marshal_xml()
    }
    /// Takes the already entity-decoded attribute value, like Go's xml.Attr.Value.
    pub fn unmarshal_xml_attr(&mut self, value: &str) -> Result<(), DecimalError> {
        let value = value.trim();
        let parsed = if value.is_empty() {
            Self::default()
        } else {
            Self::new_from_string(value).map_err(|e| e.with_kind(ErrorKind::Unmarshal))?
        };
        *self = parsed;
        Ok(())
    }
}
impl NullDecimal {
    /// Returns element character content if valid, otherwise empty text; the caller supplies the element.
    pub fn marshal_xml(&self) -> String {
        if self.valid {
            self.decimal.string()
        } else {
            String::new()
        }
    }
    /// None means omit the attribute, not an empty attribute value.
    pub fn marshal_xml_attr(&self) -> Option<String> {
        self.valid.then(|| self.decimal.string())
    }
    /// Decodes the first XML element atomically; empty content clears the nullable value.
    pub fn unmarshal_xml(&mut self, source: &str) -> Result<(), DecimalError> {
        self.unmarshal_xml_attr(&element_text(source)?)
    }
    /// Decodes an already unescaped attribute atomically; trimmed empty text clears the value.
    pub fn unmarshal_xml_attr(&mut self, value: &str) -> Result<(), DecimalError> {
        let value = value.trim();
        if value.is_empty() {
            *self = Self::default();
            return Ok(());
        }
        self.decimal.unmarshal_xml_attr(value)?;
        self.valid = true;
        Ok(())
    }
}
fn valid_char(c: char) -> bool {
    matches!(c,'\t'|'\n'|'\r'|'\u{20}'..='\u{d7ff}'|'\u{e000}'..='\u{fffd}'|'\u{10000}'..='\u{10ffff}')
}
// XML 1.0 Name syntax; quick-xml tokenization alone does not validate names.
fn valid_name(name: &[u8]) -> bool {
    fn first(c: char) -> bool {
        matches!(c, ':' | '_' | 'A'..='Z' | 'a'..='z' | '\u{c0}'..='\u{d6}' | '\u{d8}'..='\u{f6}' | '\u{f8}'..='\u{2ff}' | '\u{370}'..='\u{37d}' | '\u{37f}'..='\u{1fff}' | '\u{200c}'..='\u{200d}' | '\u{2070}'..='\u{218f}' | '\u{2c00}'..='\u{2fef}' | '\u{3001}'..='\u{d7ff}' | '\u{f900}'..='\u{fdcf}' | '\u{fdf0}'..='\u{fffd}' | '\u{10000}'..='\u{effff}')
    }
    let Ok(name) = std::str::from_utf8(name) else {
        return false;
    };
    let mut chars = name.chars();
    chars.next().is_some_and(first) && chars.all(|c| first(c) || matches!(c, '-' | '.' | '0'..='9' | '\u{b7}' | '\u{300}'..='\u{36f}' | '\u{203f}'..='\u{2040}'))
}
fn element_text(source: &str) -> Result<String, DecimalError> {
    let mut reader = Reader::from_str(source);
    reader.config_mut().check_comments = true;
    let mut depth = 0;
    let mut text = String::new();
    loop {
        match reader.read_event().map_err(|_| invalid())? {
            Event::Start(start) => {
                if !valid_name(start.name().as_ref()) {
                    return Err(invalid());
                }
                for attr in start.attributes().with_checks(false) {
                    let attr = attr.map_err(|_| invalid())?;
                    if !valid_name(attr.key.as_ref()) || attr.value.contains(&b'<') {
                        return Err(invalid());
                    }
                    let value = attr
                        .decode_and_unescape_value(reader.decoder())
                        .map_err(|_| invalid())?;
                    if !value.chars().all(valid_char) {
                        return Err(invalid());
                    }
                }
                depth += 1;
            }
            Event::Empty(start) => {
                if !valid_name(start.name().as_ref()) {
                    return Err(invalid());
                }
                for attr in start.attributes().with_checks(false) {
                    let attr = attr.map_err(|_| invalid())?;
                    if !valid_name(attr.key.as_ref()) || attr.value.contains(&b'<') {
                        return Err(invalid());
                    }
                    let value = attr
                        .decode_and_unescape_value(reader.decoder())
                        .map_err(|_| invalid())?
                        .into_owned();
                    if !value.chars().all(valid_char) {
                        return Err(invalid());
                    }
                }
                if depth == 0 {
                    return Ok(text);
                }
            }
            Event::End(_) => {
                if depth == 0 {
                    return Err(invalid());
                }
                depth -= 1;
                if depth == 0 {
                    return Ok(text);
                }
            }
            Event::Text(value) => {
                if value.as_ref().windows(3).any(|s| s == b"]]>") {
                    return Err(invalid());
                }
                let value = value.xml10_content().map_err(|_| invalid())?;
                if !value.chars().all(valid_char) {
                    return Err(invalid());
                }
                if depth == 1 {
                    text.push_str(&value);
                }
            }
            Event::CData(value) => {
                let value = value.xml10_content().map_err(|_| invalid())?;
                if !value.chars().all(valid_char) {
                    return Err(invalid());
                }
                if depth == 1 {
                    text.push_str(&value);
                }
            }
            Event::GeneralRef(value) => {
                let ch = if let Some(c) = value.resolve_char_ref().map_err(|_| invalid())? {
                    c
                } else {
                    match value.decode().map_err(|_| invalid())?.as_ref() {
                        "amp" => '&',
                        "lt" => '<',
                        "gt" => '>',
                        "apos" => '\'',
                        "quot" => '"',
                        _ => return Err(invalid()),
                    }
                };
                if !valid_char(ch) {
                    return Err(invalid());
                }
                if depth == 1 {
                    text.push(ch);
                }
            }
            Event::Eof => return Err(invalid()),
            _ => {}
        }
    }
}
