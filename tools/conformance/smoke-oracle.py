"""Exercise the independent oracle through its actual JSONL process boundary."""
import copy
import importlib.util
from pathlib import Path
import sys

spec = importlib.util.spec_from_file_location("driver", Path(__file__).with_name("run.py"))
driver = importlib.util.module_from_spec(spec)
spec.loader.exec_module(driver)
command = [sys.argv[1]]
request = {"schema_version": 1, "case_id": "exact-sum", "operation": "add", "params": {},
           "inputs": [{"coefficient": "123", "scale": "2", "initialized": True},
                      {"coefficient": "234", "scale": "2", "initialized": True}],
           "candidate": {"coefficient": "357", "scale": "2", "initialized": True}}
assert driver.verify_oracle(command, [request]) == 1
for key, value in [("coefficient", "358"), ("scale", "3"), ("initialized", False)]:
    changed = copy.deepcopy(request)
    changed["candidate"][key] = value
    result = driver.run(command, [changed])
    assert result[0]["status"] == "mismatch", result
print("Oracle JSONL checks passed: accepted correct result and rejected three state mutations")
