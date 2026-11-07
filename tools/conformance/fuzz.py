"""Bounded differential campaigns with independent numerical certificates.

Outcomes are selected before execution, so two implementations failing together
cannot satisfy a success case. --replay uses the exact saved requests, independent
of later generator changes. The time budget is checked between bounded batches.
"""
import argparse
from collections import Counter
import importlib.util
import json
import math
from pathlib import Path
import random
import time

spec = importlib.util.spec_from_file_location("driver", Path(__file__).with_name("run.py"))
driver = importlib.util.module_from_spec(spec)
spec.loader.exec_module(driver)

NUMERIC_OPERATIONS = (
    "add", "sub", "mul_prec", "quo_prec", "root_prec", "rescale",
    "ln_prec", "log2_prec", "log10_prec", "exp_prec",
)
PROTOCOLS = ("json", "yaml", "xml", "binary", "bson")


def state(coefficient, scale):
    return {"coefficient": str(coefficient), "scale": str(scale), "initialized": True}


def decoder_case(rng, malformed):
    """Generate known-valid or known-invalid syntax, with varied receiver state."""
    protocol = rng.choice(PROTOCOLS)
    nullable = protocol != "binary" and bool(rng.randrange(2))
    params = {"format": protocol, "nullable": str(nullable).lower(),
              "valid": str(bool(rng.randrange(2))).lower()}
    coefficient = rng.randint(-10**30, 10**30)
    text = str(coefficient) + "." + str(rng.randrange(10**12))
    if protocol == "binary":
        if malformed:
            # A nonempty truncated precision prefix is unambiguously invalid.
            payload = rng.randbytes(rng.randint(1, 3))
        else:
            scale = rng.randrange(41)
            magnitude = abs(coefficient)
            payload = (scale.to_bytes(4, "big") + bytes([3 if coefficient < 0 else 2])
                       + magnitude.to_bytes((magnitude.bit_length() + 7) // 8, "big"))
        params["text"] = payload.hex()
    elif protocol == "bson":
        params["type"] = "18"
        size = rng.randrange(8) if malformed else 8
        params["text"] = rng.randbytes(size).hex()
    elif protocol == "xml":
        tag = "amount" + str(rng.randrange(10000))
        params["text"] = f"<{tag}>{text}</{'wrong' if malformed else tag}>"
    elif protocol == "yaml":
        params["text"] = '[' + text if malformed else '"' + text + '"'
    else:
        params["text"] = text + 'e+' if malformed else text
    previous = state(rng.randint(-10**20, 10**20), rng.randrange(21))
    if rng.randrange(4) == 0:
        previous = {"coefficient": "0", "scale": "0", "initialized": False}
    return "decode", [previous], params, "error" if malformed else "ok"


def numeric_case(rng, operation):
    inputs = [state(rng.randint(-2**127, 2**127) or 1, rng.randrange(41)) for _ in range(2)]
    params = {"precision": str(rng.randrange(41)), "mode": str(rng.randrange(6))}
    if operation in {"root_prec", "rescale"}:
        inputs = inputs[:1]
        params["degree"] = str(rng.choice([1, 3, 5, 7]))
    elif operation in {"ln_prec", "log2_prec", "log10_prec"}:
        # Positive near-one and broad finite values, at modest certified scales.
        scale = rng.randrange(1, 13)
        coefficient = (10**scale + rng.choice([-1, 0, 1]) if rng.randrange(2)
                       else rng.randint(1, 10**24))
        inputs = [state(coefficient, scale)]
        params = {"precision": str(rng.randrange(25))}
    elif operation == "exp_prec":
        # Bound |x| <= 20 so every generated case must succeed in both budgets.
        inputs = [state(rng.randint(-20000, 20000), 3)]
        params = {"precision": str(rng.randrange(25))}
    return operation, inputs, params, "ok"


def batch(rng, offset):
    cases = []
    # Every batch covers every family, even on slow CI machines or short runs.
    families = list(NUMERIC_OPERATIONS) + ["parse", "decode", "malformed"]
    for index in range(52):
        family = families[index % len(families)]
        if family in {"decode", "malformed"}:
            operation, inputs, params, outcome = decoder_case(rng, family == "malformed")
        elif family == "parse":
            text = str(rng.randint(-10**100, 10**100)) + "." + str(rng.randrange(10**40))
            operation, inputs, params, outcome = "parse", [], {"text": text}, "ok"
        else:
            operation, inputs, params, outcome = numeric_case(rng, family)
        cases.append({"schema_version": 1, "case_id": f"fuzz-{offset+index}",
                      "operation": operation, "inputs": inputs, "params": params,
                      "expected_outcome": outcome})
    return cases


def execute(cases, args, context):
    expected = [case["expected_outcome"] for case in cases]
    requests = [{k: v for k, v in case.items() if k != "expected_outcome"} for case in cases]
    left, right = [], []
    try:
        left = driver.run([args.go], requests, args.timeout)
        right = driver.run([args.rust], requests, args.timeout)
        driver.compare(requests, left, right, expected)
        candidates = [dict(request, candidate=response["result"])
                      for request, response, outcome in zip(requests, right, expected)
                      if outcome == "ok" and request["operation"] in NUMERIC_OPERATIONS]
        driver.verify_oracle([args.oracle], candidates, args.timeout)
        return len(candidates)
    except Exception:
        Path(".cache").mkdir(exist_ok=True)
        Path(".cache/fuzz-failure.json").write_text(json.dumps(
            dict(context, cases=cases, go=left, rust=right), indent=2), encoding="utf-8")
        raise


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    for argument in ["go", "rust", "oracle"]:
        parser.add_argument("--" + argument, required=True)
    parser.add_argument("--seconds", type=float, default=5)
    parser.add_argument("--timeout", type=float, default=60)
    parser.add_argument("--seed", type=int, default=14598561)
    parser.add_argument("--replay", type=Path)
    args = parser.parse_args()
    if any(not math.isfinite(value) or value <= 0 for value in [args.seconds, args.timeout]):
        parser.error("seconds and timeout must be finite and positive")
    if args.replay:
        saved = json.loads(args.replay.read_text(encoding="utf-8"))
        # Older artifacts only contained expected-success cases.
        cases = [dict(case, expected_outcome=case.get("expected_outcome", "ok"))
                 for case in saved["cases"]]
        if not cases:
            parser.error("replay must contain at least one case")
        certified = execute(cases, args, {"replay": str(args.replay)})
        print(json.dumps({"replayed": len(cases), "certified": certified}))
        return
    rng = random.Random(args.seed)
    started, count, certified = time.monotonic(), 0, 0
    coverage = Counter()
    while count == 0 or time.monotonic() - started < args.seconds:
        cases = batch(rng, count)
        certified += execute(cases, args, {"seed": args.seed, "offset": count})
        coverage.update(case["operation"] + ":" + case["params"].get("format", "numeric")
                        + ":" + case["expected_outcome"] for case in cases)
        count += len(cases)
    report = {"seed": args.seed, "cases": count, "certified": certified,
              "elapsed_seconds": time.monotonic() - started, "coverage": dict(coverage)}
    Path(".cache").mkdir(exist_ok=True)
    Path(".cache/fuzz-report.json").write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    print(json.dumps(report))


if __name__ == "__main__":
    main()
