use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::io::{self, BufRead, Write};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct State {
    coefficient: String,
    scale: String,
    initialized: bool,
}

impl State {
    fn valid(&self) -> bool {
        let digits = self
            .coefficient
            .strip_prefix('-')
            .unwrap_or(&self.coefficient);
        let coefficient_valid = !digits.is_empty()
            && digits.bytes().all(|c| c.is_ascii_digit())
            && (digits == "0" || !digits.starts_with('0'))
            && self.coefficient != "-0";
        let scale_valid = self
            .scale
            .parse::<isize>()
            .is_ok_and(|s| s >= 0 && s.to_string() == self.scale);
        coefficient_valid
            && scale_valid
            && (self.initialized || (self.coefficient == "0" && self.scale == "0"))
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Request {
    schema_version: u32,
    case_id: String,
    operation: String,
    #[serde(default)]
    inputs: Vec<State>,
    #[serde(default)]
    params: Value,
}

fn execute(req: &Request) -> Value {
    let _ = &req.params;
    let mut response = json!({"schema_version": 1, "case_id": req.case_id,
        "outcome": "unsupported", "inputs_after": req.inputs, "error_kinds": []});
    if req.schema_version != 1 || req.case_id.is_empty() || !req.inputs.iter().all(State::valid) {
        response["outcome"] = json!("protocol_error");
    } else if req.operation == "state" {
        if req.inputs.len() != 1 {
            response["outcome"] = json!("protocol_error");
        } else {
            response["outcome"] = json!("ok");
            response["result"] = json!(req.inputs[0]);
        }
    }
    response
}

fn main() -> io::Result<()> {
    let mut stdout = io::BufWriter::new(io::stdout().lock());
    for line in io::stdin().lock().lines() {
        let line = line?;
        let request: Request = serde_json::from_str(&line)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        let result = std::panic::catch_unwind(|| execute(&request)).unwrap_or_else(|_| {
            json!({"schema_version":1,"case_id":request.case_id,
                "outcome":"panic","error_kinds":["Unclassified"],"inputs_after":request.inputs})
        });
        serde_json::to_writer(&mut stdout, &result)?;
        writeln!(&mut stdout)?;
        stdout.flush()?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn malformed_states_cannot_be_echoed_as_success() {
        for (coefficient, scale, initialized) in [
            ("-0", "0", true),
            ("01", "0", true),
            ("1", "-1", true),
            ("1", "0", false),
            ("0", "1", false),
            ("x", "0", true),
        ] {
            let req = Request {
                schema_version: 1,
                case_id: "invalid".into(),
                operation: "state".into(),
                inputs: vec![State {
                    coefficient: coefficient.into(),
                    scale: scale.into(),
                    initialized,
                }],
                params: json!({}),
            };
            assert_eq!(execute(&req)["outcome"], "protocol_error");
        }
    }
}
