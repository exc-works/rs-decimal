"""Verified whole-process batch timings, grouped by operation family (not microbenchmarks)."""
import argparse
import importlib.util
import json
from pathlib import Path
import platform
import statistics
import time
spec = importlib.util.spec_from_file_location("driver", Path(__file__).with_name("run.py"))
driver = importlib.util.module_from_spec(spec)
spec.loader.exec_module(driver)
parser = argparse.ArgumentParser()
parser.add_argument("--go", required=True)
parser.add_argument("--rust", required=True)
args = parser.parse_args()
report = {"platform": platform.platform(), "scope": "subprocess + JSONL + operations; 3 samples per family, every output verified", "families": {}}
for family in ["representation", "arithmetic", "conversion", "roots", "transcendentals", "formatting", "json", "binary", "yaml", "xml", "bson", "sql"]:
    cases = [json.loads(line) for line in Path(f"testdata/conformance/{family}.jsonl").read_text(encoding="utf-8").splitlines()]
    cases = [c for c in cases if c.pop("expected_outcome", "ok") == "ok" and all(int(s["scale"]) < 200 for s in c["inputs"]) and all(abs(int(v)) < 200 for k,v in c["params"].items() if k in {"precision", "degree"})][:128]
    if not cases:
        raise ValueError(f"empty benchmark family: {family}")
    reference = driver.run([args.go], cases, 60)
    times = {}
    for name, command in [("go", args.go), ("rust", args.rust)]:
        samples = []
        for _ in range(3):
            started = time.perf_counter()
            result = driver.run([command], cases, 60)
            elapsed = time.perf_counter()-started
            driver.compare(cases, reference, result)
            samples.append(elapsed)
        times[name] = {"samples_seconds": samples, "median_seconds": statistics.median(samples)}
    report["families"][family] = {"cases": len(cases), "timing": times}
Path(".cache").mkdir(exist_ok=True)
Path(".cache/benchmark.json").write_text(json.dumps(report, indent=2)+"\n", encoding="utf-8")
print(json.dumps(report))
