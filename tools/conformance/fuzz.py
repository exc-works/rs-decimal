"""Reproducible bounded differential fuzzing with independent arithmetic certificates.

--seconds selects a minimum campaign duration. Each bounded batch has a timeout;
failures retain the seed, complete input and both outputs for exact reproduction.
"""
import argparse
import importlib.util
import json
from pathlib import Path
import random
import time

spec = importlib.util.spec_from_file_location("driver", Path(__file__).with_name("run.py"))
driver = importlib.util.module_from_spec(spec)
spec.loader.exec_module(driver)


def batch(rng, offset):
    cases = []
    for i in range(128):
        op = rng.choice(["add", "sub", "mul_prec", "quo_prec", "root_prec", "rescale", "parse", "decode"])
        inputs = [{"coefficient": str(rng.randint(-2**127, 2**127) or 1), "scale": str(rng.randrange(41)), "initialized": True} for _ in range(2)]
        params = {"precision": str(rng.randrange(41)), "mode": str(rng.randrange(6))}
        if op in {"root_prec", "rescale"}:
            inputs = inputs[:1]
            params["degree"] = str(rng.choice([1, 3, 5, 7]))
        elif op in {"parse", "decode"}:
            digits = str(rng.randint(1, 10**100))
            text = ("-" if rng.randrange(2) else "") + digits + "." + str(rng.randint(0, 10**40)) + "e" + str(rng.randint(-100, 100))
            if op == "parse":
                inputs, params = [], {"text": text}
            else:
                inputs = inputs[:1]
                params = {"nullable": rng.choice(["true", "false"]), "valid": rng.choice(["true", "false"]), "format": "json", "text": text}
        cases.append({"schema_version": 1, "case_id": f"fuzz-{offset+i}", "operation": op, "inputs": inputs, "params": params})
    return cases


def main():
    parser = argparse.ArgumentParser()
    for arg in ["go", "rust", "oracle"]:
        parser.add_argument("--" + arg, required=True)
    parser.add_argument("--seconds", type=float, default=5)
    parser.add_argument("--seed", type=int, default=14598561)
    args = parser.parse_args()
    if args.seconds <= 0:
        parser.error("seconds must be positive")
    rng = random.Random(args.seed)
    started, count, certified = time.monotonic(), 0, 0
    while count == 0 or time.monotonic()-started < args.seconds:
        cases, left, right = batch(rng, count), [], []
        try:
            left, right = driver.run([args.go], cases, 60), driver.run([args.rust], cases, 60)
            driver.compare(cases, left, right)
            candidates = [dict(c, candidate=r["result"]) for c, r in zip(cases, right) if c["operation"] not in {"parse", "decode"}]
            driver.verify_oracle([args.oracle], candidates, 60)
            certified += len(candidates)
        except Exception:
            Path(".cache").mkdir(exist_ok=True)
            Path(".cache/fuzz-failure.json").write_text(json.dumps({"seed": args.seed, "offset": count, "cases": cases, "go": left, "rust": right}, indent=2), encoding="utf-8")
            raise
        count += len(cases)
    report = {"seed": args.seed, "cases": count, "certified": certified, "elapsed_seconds": time.monotonic()-started}
    Path(".cache").mkdir(exist_ok=True)
    Path(".cache/fuzz-report.json").write_text(json.dumps(report, indent=2)+"\n", encoding="utf-8")
    print(json.dumps(report))


if __name__ == "__main__":
    main()
