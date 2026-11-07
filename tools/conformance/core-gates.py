"""Core corpus, deterministic differential fuzz smoke, and verified batch timing.

Timing includes process startup and JSONL transport, not just arithmetic. Native
scale fixtures are instantiated at the selected paired Go/Rust pointer width.
"""
import argparse
import importlib.util
import json
from pathlib import Path
import platform
import random
import statistics
import time

spec = importlib.util.spec_from_file_location("driver", Path(__file__).with_name("run.py"))
driver = importlib.util.module_from_spec(spec)
spec.loader.exec_module(driver)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--go", required=True)
    parser.add_argument("--rust", required=True)
    parser.add_argument("--oracle", required=True)
    parser.add_argument("--native-bits", type=int, choices=[32, 64], default=64)
    parser.add_argument("--report", default=".cache/core-gates.json")
    args = parser.parse_args()
    total = 0
    for path in sorted(Path("testdata/conformance").glob("*.jsonl")):
        cases = [json.loads(line) for line in path.read_text(encoding="utf-8").splitlines() if line.strip()]
        expected = [c.pop("expected_outcome", "ok") for c in cases]
        for c in cases:
            for state in c["inputs"]:
                if state["scale"] == str(2**63-1):
                    state["scale"] = str(2**(args.native_bits-1)-1)
        driver.compare(cases, driver.run([args.go], cases, 60), driver.run([args.rust], cases, 60), expected)
        total += len(cases)
    rng = random.Random(0xDEC1A1)
    fuzz = []
    for i in range(512):
        op = rng.choice(["add", "sub", "mul_prec", "quo_prec", "root_prec"])
        def state():
            return {"coefficient": str(rng.randint(-10**18, 10**18) or 1), "scale": str(rng.randrange(19)), "initialized": True}
        inputs = [state(), state()]
        params = {"precision": str(rng.randrange(20)), "mode": str(rng.randrange(6))}
        if op == "root_prec":
            inputs = inputs[:1]
            params["degree"] = str(rng.choice([1, 3, 5, 7]))
        fuzz.append({"schema_version": 1, "case_id": f"seed-dec1a1-{i}", "operation": op, "inputs": inputs, "params": params})
    left = driver.run([args.go], fuzz, 60)
    right = driver.run([args.rust], fuzz, 60)
    try:
        driver.compare(fuzz, left, right)
        driver.verify_oracle([args.oracle], [dict(c, candidate=r["result"]) for c, r in zip(fuzz, right)], 60)
    except Exception:
        Path(".cache").mkdir(exist_ok=True)
        Path(".cache/core-fuzz-failure.json").write_text(json.dumps({"cases": fuzz, "go": left, "rust": right}, indent=2), encoding="utf-8")
        raise
    times = {}
    for name, command in [("go", args.go), ("rust", args.rust)]:
        samples = []
        for _ in range(3):
            start = time.perf_counter()
            result = driver.run([command], fuzz, 60)
            samples.append(time.perf_counter()-start)
            driver.compare(fuzz, left, result)
        times[name] = {"samples_seconds": samples, "median_seconds": statistics.median(samples)}
    report = {"platform": platform.platform(), "native_bits": args.native_bits, "corpus_cases": total,
              "fuzz_cases": len(fuzz), "seed": "0xDEC1A1", "all_results_verified": True,
              "timing_scope": "512 mixed operations, subprocess startup and JSONL transport included", "timing": times}
    path = Path(args.report)
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(report, indent=2)+"\n", encoding="utf-8")
    print(json.dumps(report))


if __name__ == "__main__":
    main()
