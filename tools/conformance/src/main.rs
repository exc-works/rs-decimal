use rs_decimal::{BigInt, BigRational, Decimal, DecimalError, RoundingMode};
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
    fn decimal(&self) -> Decimal {
        if self.initialized {
            Decimal::new_from_big_int_with_prec(
                &self.coefficient.parse::<BigInt>().unwrap(),
                self.scale.parse().unwrap(),
            )
        } else {
            Decimal::default()
        }
    }
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
    let mut response = json!({"schema_version": 1, "case_id": req.case_id,
        "outcome": "unsupported", "inputs_after": req.inputs, "error_kinds": []});
    if req.schema_version != 1 || req.case_id.is_empty() || !req.inputs.iter().all(State::valid) {
        response["outcome"] = json!("protocol_error");
    } else if req.operation == "state" {
        if req.inputs.len() != 1 {
            response["outcome"] = json!("protocol_error");
        } else {
            response["outcome"] = json!("ok");
            response["result"] = capture(&req.inputs[0].decimal());
        }
    } else {
        let values: Vec<_> = req.inputs.iter().map(State::decimal).collect();
        let p = |key: &str| req.params[key].as_str().expect("missing string parameter");
        let result: Result<Value, DecimalError> = match req.operation.as_str() {
            "format" if values.len() == 1 => {
                let flags = p("flags");
                let f = rs_decimal::GoFormat {
                    verb: p("verb").chars().next().unwrap(),
                    width: req
                        .params
                        .get("width")
                        .and_then(|v| v.as_str())
                        .map(|v| v.parse().unwrap()),
                    precision: req
                        .params
                        .get("precision")
                        .and_then(|v| v.as_str())
                        .map(|v| v.parse().unwrap()),
                    left: flags.contains('-'),
                    plus: flags.contains('+'),
                    space: flags.contains(' '),
                    zero: flags.contains('0'),
                    alternate: flags.contains('#'),
                };
                Ok(json!(values[0].format_go(f)))
            }
            "string" if values.len() == 1 => Ok(json!(values[0].string())),
            "trailing" if values.len() == 1 => Ok(json!(values[0].string_with_trailing_zeros())),
            "separators" if values.len() == 1 => Ok(json!(values[0].format_with_separators(
                p("thousands").chars().next().unwrap(),
                p("decimal").chars().next().unwrap()
            ))),
            "append" if values.len() == 1 => {
                let mut out = p("prefix").as_bytes().to_vec();
                if p("trailing") == "true" {
                    values[0].append_text_with_trailing_zeros(&mut out);
                } else {
                    values[0].append_text(&mut out);
                }
                Ok(json!(String::from_utf8(out).unwrap()))
            }
            "ln" | "log2" | "log10" | "exp" | "ln_prec" | "log2_prec" | "log10_prec"
            | "exp_prec"
                if values.len() == 1 =>
            {
                let a = &values[0];
                let result = match req.operation.as_str() {
                    "ln" => a.ln(),
                    "ln_prec" => a.ln_with_prec(p("precision").parse().unwrap()),
                    "log2" => Ok(a.log2()),
                    "log2_prec" => a.log2_with_prec(p("precision").parse().unwrap()),
                    "log10" => a.log10(),
                    "log10_prec" => a.log10_with_prec(p("precision").parse().unwrap()),
                    "exp" => a.exp(),
                    "exp_prec" => a.exp_with_prec(p("precision").parse().unwrap()),
                    _ => unreachable!(),
                };
                result.map(|d| capture(&d))
            }
            "sqrt" | "root" | "sqrt_prec" | "root_prec" if values.len() == 1 => {
                let a = &values[0];
                let degree = req.params["degree"]
                    .as_str()
                    .unwrap_or("2")
                    .parse()
                    .unwrap();
                let result = match req.operation.as_str() {
                    "sqrt" => a.sqrt(),
                    "root" => a.approx_root(degree),
                    "sqrt_prec" => a.sqrt_with_prec(p("precision").parse().unwrap()),
                    _ => a.approx_root_with_prec(degree, p("precision").parse().unwrap()),
                };
                result.map(|d| capture(&d))
            }
            "from_float" => {
                let bits = u64::from_str_radix(p("bits"), 16).unwrap();
                let d = if p("width") == "32" {
                    Decimal::new_from_float32(f32::from_bits(bits as u32))
                } else {
                    Decimal::new_from_float64(f64::from_bits(bits))
                };
                Ok(capture(&d))
            }
            "to_float" if values.len() == 1 => {
                let (bits, exact) = if p("width") == "32" {
                    let (v, e) = values[0].float32();
                    (format!("{:08x}", v.to_bits()), e)
                } else {
                    let (v, e) = values[0].float64();
                    (format!("{:016x}", v.to_bits()), e)
                };
                Ok(json!({"width":p("width"),"bits":bits,"exact":exact}))
            }
            "from_rat" | "from_rat_prec" => {
                let r = BigRational::new(
                    p("numerator").parse().unwrap(),
                    p("denominator").parse().unwrap(),
                );
                let d = if req.operation == "from_rat" {
                    Decimal::new_from_big_rat(&r)
                } else {
                    Decimal::new_from_big_rat_with_prec(
                        &r,
                        p("precision").parse().unwrap(),
                        RoundingMode::from_raw(p("mode").parse().unwrap()),
                    )
                };
                d.map(|d| capture(&d))
            }
            "to_rat" if values.len() == 1 => {
                let r = values[0].big_rat();
                Ok(json!({"numerator":r.numer().to_string(),"denominator":r.denom().to_string()}))
            }
            "native_int" if values.len() == 1 => {
                let (a, ea) = values[0].int64();
                let (b, eb) = values[0].uint64();
                Ok(
                    json!({"signed":a.to_string(),"signed_exact":ea,"unsigned":b.to_string(),"unsigned_exact":eb,"bit_len":values[0].bit_len().to_string()}),
                )
            }
            "msb" => Ok(json!(
                rs_decimal::most_significant_bit(&p("value").parse().unwrap()).to_string()
            )),
            "add" | "sub" | "safe_add" | "safe_sub" | "mul" | "mul_down" | "mul_exact"
            | "mul_prec" | "quo" | "quo_down" | "quo_prec" | "quo_rem" | "mod"
                if values.len() == 2 =>
            {
                let (a, b) = (&values[0], &values[1]);
                let mode = RoundingMode::from_raw(
                    req.params["mode"].as_str().unwrap_or("5").parse().unwrap(),
                );
                let prec = req.params["precision"]
                    .as_str()
                    .unwrap_or("0")
                    .parse()
                    .unwrap();
                let result = match req.operation.as_str() {
                    "add" => a.add(b),
                    "sub" => a.sub(b),
                    "safe_add" => a.safe_add(b),
                    "safe_sub" => a.safe_sub(b),
                    "mul" => a.mul(b, mode),
                    "mul_down" => a.mul_down(b),
                    "mul_exact" => a.mul_exact(b),
                    "mul_prec" => a.mul_with_prec(b, prec, mode),
                    "quo" => a.quo(b, mode),
                    "quo_down" => a.quo_down(b),
                    "quo_prec" => a.quo_with_prec(b, prec, mode),
                    "mod" => a.r#mod(b),
                    "quo_rem" => {
                        let (q, r) = a.quo_rem(b);
                        return json!({"schema_version":1,"case_id":req.case_id,"outcome":"ok","error_kinds":[],"inputs_after":req.inputs,"result":[capture(&q),capture(&r)]});
                    }
                    _ => unreachable!(),
                };
                Ok(capture(&result))
            }
            "power" | "add_raw" | "sub_raw" | "must_non_negative" | "integer_parts"
                if values.len() == 1 =>
            {
                let a = &values[0];
                let value = req.params["value"].as_str().unwrap_or("0").parse().unwrap();
                match req.operation.as_str() {
                    "power" => Ok(capture(&a.power(value))),
                    "add_raw" => Ok(capture(&a.add_raw(value))),
                    "sub_raw" => Ok(capture(&a.sub_raw(value))),
                    "must_non_negative" => Ok(capture(&a.must_non_negative())),
                    "integer_parts" => {
                        let (i, f) = a.remainder();
                        Ok(
                            json!({"int_part":a.int_part().to_string(),"integer":i.to_string(),"fraction":f.to_string(),"is_integer":a.is_integer(),"has_fraction":a.has_fraction()}),
                        )
                    }
                    _ => unreachable!(),
                }
            }
            "mode" => Ok(json!(
                (RoundingMode::from_raw(p("mode").parse().unwrap()) as i32).to_string()
            )),
            "rescale" | "rescale_down" | "floor" | "ceil" | "round" | "truncate" | "shift"
            | "strip" | "significant"
                if values.len() == 1 =>
            {
                let a = &values[0];
                let prec = req.params["precision"]
                    .as_str()
                    .unwrap_or("0")
                    .parse()
                    .unwrap();
                let result = match req.operation.as_str() {
                    "rescale" => {
                        a.rescale(prec, RoundingMode::from_raw(p("mode").parse().unwrap()))
                    }
                    "rescale_down" => a.rescale_down(prec),
                    "floor" => a.floor_with_prec(prec),
                    "ceil" => a.ceil_with_prec(prec),
                    "round" => a.round_with_prec(prec),
                    "truncate" => a.truncate_with_prec(prec),
                    "shift" => a.shift(p("places").parse().unwrap()),
                    "strip" => a.strip_trailing_zeros(),
                    "significant" => a.significant_figures(
                        p("figures").parse().unwrap(),
                        RoundingMode::from_raw(p("mode").parse().unwrap()),
                    ),
                    _ => unreachable!(),
                };
                Ok(capture(&result))
            }
            "parse" => Decimal::new_from_string(p("text")).map(|d| capture(&d)),
            "must_parse" => Ok(capture(&Decimal::must_from_string(p("text")))),
            "construct" => {
                let prec = req.params["precision"]
                    .as_str()
                    .unwrap_or("0")
                    .parse()
                    .unwrap();
                let d = match p("kind") {
                    "new" => Decimal::new(p("value").parse().unwrap()),
                    "int" => Decimal::new_from_int(p("value").parse().unwrap()),
                    "with_prec" => Decimal::new_with_prec(p("value").parse().unwrap(), prec),
                    "int64" => Decimal::new_from_int64(p("value").parse().unwrap(), prec),
                    "uint64" => Decimal::new_from_uint64(p("value").parse().unwrap(), prec),
                    "big_int" => Decimal::new_from_big_int(&p("value").parse().unwrap()),
                    "big_int_prec" => {
                        Decimal::new_from_big_int_with_prec(&p("value").parse().unwrap(), prec)
                    }
                    "append" => Decimal::new_with_append_prec(p("value").parse().unwrap(), prec),
                    "uint_append" => {
                        Decimal::new_from_uint_with_append_prec(p("value").parse().unwrap(), prec)
                    }
                    _ => return response,
                };
                Ok(capture(&d))
            }
            "inspect" if values.len() == 1 => {
                let d = &values[0];
                Ok(
                    json!({"sign":d.sign().to_string(),"nil":d.is_nil(),"zero":d.is_zero(),"not_zero":d.is_not_zero(),"positive":d.is_positive(),"negative":d.is_negative(),"precision":d.precision().to_string()}),
                )
            }
            "cmp" if values.len() == 2 => {
                let (a, b) = (&values[0], &values[1]);
                Ok(
                    json!({"cmp":a.cmp(b).to_string(),"equal":a.equal(b),"not_equal":a.not_equal(b),"gt":a.gt(b),"gte":a.gte(b),"lt":a.lt(b),"lte":a.lte(b)}),
                )
            }
            "max" if values.len() == 2 => Ok(capture(&Decimal::max(&values[0], &values[1]))),
            "min" if values.len() == 2 => Ok(capture(&Decimal::min(&values[0], &values[1]))),
            "between" if values.len() == 3 => Ok(json!(rs_decimal::between(
                &values[0], &values[1], &values[2]
            ))),
            "clone" if values.len() == 1 => Ok(capture(&Decimal::new_from_decimal(&values[0]))),
            "abs" if values.len() == 1 => Ok(capture(&values[0].abs())),
            "neg" if values.len() == 1 => Ok(capture(&values[0].neg())),
            _ => return response,
        };
        match result {
            Ok(value) => {
                response["outcome"] = json!("ok");
                response["result"] = value;
            }
            Err(e) => {
                response["outcome"] = json!("error");
                response["error_kinds"] = json!(
                    e.kinds()
                        .iter()
                        .map(|k| format!("{k:?}"))
                        .collect::<Vec<_>>()
                );
            }
        }
    }
    response
}

fn capture(d: &Decimal) -> Value {
    json!({"coefficient":d.big_int().to_string(),"scale":d.precision().to_string(),"initialized":!d.is_nil()})
}

fn main() -> io::Result<()> {
    let mut stdout = io::BufWriter::new(io::stdout().lock());
    for line in io::stdin().lock().lines() {
        let line = line?;
        let request: Request = serde_json::from_str(&line)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        let result = std::panic::catch_unwind(|| execute(&request)).unwrap_or_else(|panic| {
            let kinds = if let Some(e) = panic.downcast_ref::<DecimalError>() {
                e.kinds()
                    .iter()
                    .map(|k| format!("{k:?}"))
                    .collect::<Vec<_>>()
            } else {
                vec!["Assertion".into()]
            };
            json!({"schema_version":1,"case_id":request.case_id,
                "outcome":"panic","error_kinds":kinds,"inputs_after":request.inputs})
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
