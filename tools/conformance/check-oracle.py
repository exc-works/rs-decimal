"""Certify selected successful Rust results using the independent Go oracle."""
import argparse
import importlib.util
import json
from pathlib import Path

spec = importlib.util.spec_from_file_location("driver", Path(__file__).with_name("run.py"))
driver = importlib.util.module_from_spec(spec)
spec.loader.exec_module(driver)
parser = argparse.ArgumentParser()
parser.add_argument("--rust", required=True)
parser.add_argument("--oracle", required=True)
parser.add_argument("--cases", required=True)
parser.add_argument("--operations", nargs="+", required=True)
parser.add_argument("--timeout", type=float, default=60)
args = parser.parse_args()
cases = [json.loads(line) for line in Path(args.cases).read_text().splitlines() if line.strip()]
selected = [c for c in cases if c["operation"] in args.operations and c.get("expected_outcome", "ok") == "ok"]
if not selected:
    raise driver.ConformanceError("empty oracle selection")
for c in selected:
    c.pop("expected_outcome", None)
responses = driver.run([args.rust], selected, args.timeout)
requests = []
for req, response in zip(selected, responses):
    if response.get("outcome") != "ok":
        raise driver.ConformanceError(f"unexpected Rust failure in oracle case {req['case_id']}")
    driver.validate_state(response.get("result"))
    requests.append(dict(req, candidate=response["result"]))
print(f"Independent oracle verified {driver.verify_oracle([args.oracle], requests, args.timeout)} Rust results")
