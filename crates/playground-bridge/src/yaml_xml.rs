use super::{Decimal, NullDecimal, library_error, limits, resource_error, tick};
use quick_xml::{Reader, events::Event as XmlEvent};
use wasm_bindgen::prelude::*;
use yaml_rust2::parser::{Event as YamlEvent, Parser};

fn source_budget(source: &str) -> Result<(), &'static str> {
    if source.len() > limits::INPUT_BYTES {
        return Err("playground document input exceeds 8192 UTF-8 bytes");
    }
    Ok(())
}
fn yaml_budget(source: &str) -> Result<(), &'static str> {
    source_budget(source)?;
    let mut parser = Parser::new_from_str(source);
    let mut docs = 0;
    loop {
        match parser.next_token() {
            Ok((YamlEvent::DocumentStart, _)) => docs += 1,
            Ok((YamlEvent::Scalar(text, _, _, _), _)) if docs <= 1 => {
                // Core removes underscores for resolved numeric scalars and may parse
                // radix integers. Bound all candidate digits before that allocation.
                let clean = text.replace('_', "");
                if clean.chars().filter(char::is_ascii_hexdigit).count() > 4096 {
                    return Err("playground YAML scalar exceeds coefficient budget");
                }
                limits::input(&clean)?;
                // YAML float resolution inserts a fractional zero for `1.`.
                // Check that representation too (e.g. `!!float 1.e-1000`).
                let (mantissa, exponent) = clean
                    .find(['e', 'E'])
                    .map_or((clean.as_str(), ""), |i| (&clean[..i], &clean[i..]));
                if mantissa.ends_with('.') {
                    limits::input(&format!("{mantissa}0{exponent}"))?;
                }
            }
            Ok((YamlEvent::StreamEnd, _)) | Err(_) => return Ok(()),
            _ => {}
        }
    }
}
fn xml_budget(source: &str) -> Result<(), &'static str> {
    source_budget(source)?;
    let mut reader = Reader::from_str(source);
    let mut depth = 0usize;
    let mut text = String::new();
    // Collect the same first-element direct text as the core. Syntax validation
    // remains the core's job; parser errors cannot lead to decimal allocation.
    loop {
        match reader.read_event() {
            Ok(XmlEvent::Start(_)) => depth += 1,
            Ok(XmlEvent::Empty(_)) if depth == 0 => break,
            Ok(XmlEvent::End(_)) => {
                if depth == 0 {
                    return Ok(());
                }
                depth -= 1;
                if depth == 0 {
                    break;
                }
            }
            Ok(XmlEvent::Text(value)) if depth == 1 => {
                let Ok(value) = value.xml10_content() else {
                    return Ok(());
                };
                text.push_str(&value);
            }
            Ok(XmlEvent::CData(value)) if depth == 1 => {
                let Ok(value) = value.xml10_content() else {
                    return Ok(());
                };
                text.push_str(&value);
            }
            Ok(XmlEvent::GeneralRef(value)) if depth == 1 => {
                let ch = match value.resolve_char_ref() {
                    Ok(Some(ch)) => ch,
                    Ok(None) => match value.decode().as_deref() {
                        Ok("amp") => '&',
                        Ok("lt") => '<',
                        Ok("gt") => '>',
                        Ok("apos") => '\'',
                        Ok("quot") => '"',
                        _ => return Ok(()),
                    },
                    Err(_) => return Ok(()),
                };
                text.push(ch);
            }
            Ok(XmlEvent::Eof) | Err(_) => return Ok(()),
            _ => {}
        }
    }
    limits::input(&text)
}
#[wasm_bindgen]
impl Decimal {
    pub fn marshal_yaml(&self) -> Result<String, JsValue> {
        tick()?;
        Ok(self.inner.marshal_yaml())
    }
    pub fn unmarshal_yaml(&mut self, source: &str) -> Result<(), JsValue> {
        tick()?;
        yaml_budget(source).map_err(resource_error)?;
        self.inner.unmarshal_yaml(source).map_err(library_error)
    }
    pub fn marshal_xml(&self) -> Result<String, JsValue> {
        tick()?;
        Ok(self.inner.marshal_xml())
    }
    pub fn marshal_xml_attr(&self) -> Result<String, JsValue> {
        tick()?;
        Ok(self.inner.marshal_xml_attr())
    }
    pub fn unmarshal_xml(&mut self, source: &str) -> Result<(), JsValue> {
        tick()?;
        xml_budget(source).map_err(resource_error)?;
        self.inner.unmarshal_xml(source).map_err(library_error)
    }
    pub fn unmarshal_xml_attr(&mut self, value: &str) -> Result<(), JsValue> {
        tick()?;
        limits::input(value).map_err(resource_error)?;
        self.inner.unmarshal_xml_attr(value).map_err(library_error)
    }
}
#[wasm_bindgen]
impl NullDecimal {
    pub fn marshal_yaml(&self) -> Result<Option<String>, JsValue> {
        tick()?;
        Ok(self.inner.marshal_yaml())
    }
    pub fn unmarshal_yaml(&mut self, source: &str) -> Result<(), JsValue> {
        tick()?;
        yaml_budget(source).map_err(resource_error)?;
        self.inner.unmarshal_yaml(source).map_err(library_error)
    }
    pub fn marshal_xml(&self) -> Result<String, JsValue> {
        tick()?;
        Ok(self.inner.marshal_xml())
    }
    pub fn marshal_xml_attr(&self) -> Result<Option<String>, JsValue> {
        tick()?;
        Ok(self.inner.marshal_xml_attr())
    }
    pub fn unmarshal_xml(&mut self, source: &str) -> Result<(), JsValue> {
        tick()?;
        xml_budget(source).map_err(resource_error)?;
        self.inner.unmarshal_xml(source).map_err(library_error)
    }
    pub fn unmarshal_xml_attr(&mut self, value: &str) -> Result<(), JsValue> {
        tick()?;
        limits::input(value).map_err(resource_error)?;
        self.inner.unmarshal_xml_attr(value).map_err(library_error)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn decoded_documents_cannot_hide_exponent_or_radix_growth() {
        for source in ["\"\\u0031e100000\"", "!!float 1.0e1_000_000", "'1e-100000'"] {
            assert!(yaml_budget(source).is_err());
        }
        assert!(yaml_budget(&format!("!!int 0x{}", "f".repeat(4097))).is_err());
        assert!(yaml_budget("1.25\n---\n1e100000").is_ok());
        assert!(xml_budget("<d>&#49;e100000</d>").is_err());
        assert!(xml_budget("<d>1<![CDATA[e100000]]></d>").is_err());
        assert!(xml_budget("<d>1<x>e100000</x>.25</d>").is_ok());
        assert!(xml_budget("<d>1.25</d><d>1e100000</d>").is_ok());
        assert!(xml_budget("<d>&unknown;</d>").is_ok());
    }
}
