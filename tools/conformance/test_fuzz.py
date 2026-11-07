"""Guard the campaign's evidence: deterministic requests, outcome checks and replay."""
import argparse
import importlib.util
from pathlib import Path
import random
import tempfile
import unittest
from unittest.mock import patch

spec = importlib.util.spec_from_file_location("fuzz", Path(__file__).with_name("fuzz.py"))
fuzz = importlib.util.module_from_spec(spec)
spec.loader.exec_module(fuzz)


class CampaignTests(unittest.TestCase):
    def test_seed_and_success_family_coverage(self):
        cases = fuzz.batch(random.Random(42), 0)
        self.assertEqual(cases, fuzz.batch(random.Random(42), 0))
        self.assertNotEqual(cases, fuzz.batch(random.Random(43), 0))
        self.assertTrue(set(fuzz.NUMERIC_OPERATIONS) <= {c["operation"] for c in cases})
        self.assertEqual({c["expected_outcome"] for c in cases}, {"ok", "error"})

    def test_joint_failure_cannot_satisfy_success(self):
        case = fuzz.batch(random.Random(42), 0)[0]
        response = {"schema_version": 1, "case_id": case["case_id"], "outcome": "error",
                    "error_kinds": ["NoConvergence"], "inputs_after": case["inputs"]}
        args = argparse.Namespace(go="go", rust="rust", oracle="oracle", timeout=1)
        with tempfile.TemporaryDirectory() as directory:
            with patch.object(fuzz.driver, "run", return_value=[response]), \
                 patch.object(fuzz, "Path", side_effect=lambda name: Path(directory) / name):
                with self.assertRaises(fuzz.driver.ConformanceError):
                    fuzz.execute([case], args, {"seed": 42})
                self.assertTrue((Path(directory) / ".cache/fuzz-failure.json").exists())


if __name__ == "__main__":
    unittest.main()
