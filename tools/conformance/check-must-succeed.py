"""Run the frozen promised-success corpus against Rust and independent certificates."""
import argparse
import sys
sys.set_int_max_str_digits(0)
from decimal import Decimal
import importlib.util
import json
from pathlib import Path

spec = importlib.util.spec_from_file_location("driver", Path(__file__).with_name("run.py"))
driver = importlib.util.module_from_spec(spec)
spec.loader.exec_module(driver)
ROOT = Path(__file__).resolve().parents[2]


def parts(text):
    sign, digits, exponent = Decimal(text).as_tuple()
    coefficient = int("".join(str(d) for d in digits)) * (-1 if sign else 1)
    if exponent > 0:
        coefficient *= 10**exponent
    return coefficient, max(0, -exponent)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--rust", required=True)
    parser.add_argument("--oracle", required=True)
    parser.add_argument("--operations", nargs="+", default=["sqrt", "root", "ln", "log2", "log10", "exp"])
    args = parser.parse_args()
    corpus = json.loads((ROOT/"testdata/upstream/testdata/must_succeed.json").read_text(encoding="utf-8"))
    cases = [c for c in corpus["cases"] if c["op"] in args.operations]
    assert cases, "empty promised-success selection"
    requests = []
    for c in cases:
        coefficient, scale = parts(c["input"])
        if coefficient == 0:
            scale = 0
        operation = c["op"] + ("" if c.get("default") else "_prec")
        requests.append({"schema_version": 1, "case_id": c["id"], "operation": operation,
                         "inputs": [{"coefficient": str(coefficient), "scale": str(scale), "initialized": True}],
                         "params": {"precision": str(c["prec"]), "degree": str(c.get("degree", 2))}})
    responses = driver.run([args.rust], requests, 60)
    candidates = []
    for c, req, response in zip(cases, requests, responses):
        assert response["outcome"] == "ok", (c["id"], response)
        driver.validate_state(response["result"])
        want, want_scale = parts(c["want"])
        assert want_scale <= c["prec"]
        want *= 10**(c["prec"]-want_scale)
        assert response["result"] == {"coefficient": str(want), "scale": str(c["prec"]), "initialized": True}, c["id"]
        assert response["inputs_after"] == req["inputs"], "mutated input"
        candidates.append(dict(req, candidate=response["result"]))
    driver.verify_oracle([args.oracle], candidates, 60)
    print(f"{len(cases)} fixed must-succeed cases passed exact output/state and independent certification")


if __name__ == "__main__":
    main()
