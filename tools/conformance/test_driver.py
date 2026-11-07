import copy
import json
import importlib.util
from pathlib import Path
import sys
import unittest

spec = importlib.util.spec_from_file_location("driver", Path(__file__).with_name("run.py"))
driver = importlib.util.module_from_spec(spec)
spec.loader.exec_module(driver)


class DriverTests(unittest.TestCase):
    def setUp(self):
        self.request = {"schema_version": 1, "case_id": "a", "operation": "state", "inputs": []}
        self.response = {"schema_version": 1, "case_id": "a", "outcome": "ok", "error_kinds": [], "inputs_after": [],
                         "result": {"coefficient": "100", "scale": "2", "initialized": True}}

    def test_detects_all_state_dimensions(self):
        for key, value in [("coefficient", "101"), ("scale", "3"), ("initialized", False)]:
            changed = copy.deepcopy(self.response)
            changed["result"][key] = value
            with self.subTest(key=key), self.assertRaises(driver.ConformanceError):
                driver.compare([self.request], [self.response], [changed])
        changed = copy.deepcopy(self.response)
        changed["inputs_after"] = [changed["result"]]
        with self.assertRaises(driver.ConformanceError):
            driver.compare([self.request], [self.response], [changed])

    def test_error_categories_and_decoder_states(self):
        original = dict(self.response, outcome="error", error_kinds=["Unmarshal", "InvalidFormat"], receiver_before={}, receiver_after={})
        for field, value in [("error_kinds", ["Unmarshal"]), ("receiver_after", {"valid": True})]:
            changed = dict(original, **{field: value})
            with self.subTest(field=field), self.assertRaises(driver.ConformanceError):
                driver.compare([self.request], [original], [changed], ["error"])

    def test_same_unexpected_failure_is_not_success(self):
        for outcome in ["error", "panic", "unsupported", "protocol_error"]:
            response = dict(self.response, outcome=outcome)
            with self.subTest(outcome=outcome), self.assertRaises(driver.ConformanceError):
                driver.compare([self.request], [response], [response])

    def test_transport_failures(self):
        for code in ["raise SystemExit(2)", "print('diagnostic on stdout')", "pass", "print('{}')", "import time; time.sleep(2)"]:
            with self.subTest(code=code), self.assertRaises(driver.ConformanceError):
                driver.run([sys.executable, "-c", code], [self.request], timeout=0.1)

    def test_error_order_and_valid_response(self):
        a = dict(self.response, outcome="error", error_kinds=["InvalidFormat", "Unmarshal"])
        b = dict(a, error_kinds=["Unmarshal", "InvalidFormat"])
        self.assertEqual(driver.compare([self.request], [a], [b], ["error"]), {"error": 1})

    def test_oracle_refuses_uncertified(self):
        code = "print('{\"schema_version\":1,\"case_id\":\"a\",\"status\":\"uncertified\"}')"
        with self.assertRaises(driver.ConformanceError):
            driver.verify_oracle([sys.executable, "-c", code], [self.request])

    def test_both_executors_omitting_state_is_rejected(self):
        req = dict(self.request, inputs=[self.response["result"]])
        for fields in [{"inputs_after": [], "result": {}},
                       {"inputs_after": [{}]},
                       {"inputs_after": [self.response["result"]], "result": {}},
                       {"inputs_after": [dict(self.response["result"], initialized=1)]}]:
            response = dict(self.response, **fields)
            with self.subTest(fields=fields), self.assertRaises(driver.ConformanceError):
                driver.compare([req], [response], [response])

    def test_decode_requires_complete_state_even_on_matching_failures(self):
        state = self.response["result"]
        req = dict(self.request, operation="decode", inputs=[state], params={"nullable": "true", "valid": "false"})
        response = dict(self.response, outcome="error", inputs_after=[state], error_kinds=["Unmarshal"], receiver_after={"decimal": state, "valid": False})
        self.assertEqual(driver.compare([req], [response], [response], ["error"]), {"error": 1})
        for bad in [{}, {"decimal": state}, {"decimal": state, "valid": True}, {"decimal": dict(state, scale="3"), "valid": False}]:
            mutated = dict(response, receiver_after=bad)
            with self.subTest(bad=bad), self.assertRaises(driver.ConformanceError):
                driver.compare([req], [mutated], [mutated], ["error"])

    def test_noncanonical_states_are_rejected(self):
        for fields in [{"coefficient": "+0100"}, {"scale": "02"}, {"coefficient": "-0"}, {"scale": 2}]:
            with self.subTest(fields=fields), self.assertRaises(driver.ConformanceError):
                driver.validate_state(dict(self.response["result"], **fields))


class Utf8TransportTests(unittest.TestCase):
    def test_non_ascii_protocol_is_independent_of_system_encoding(self):
        from unittest.mock import patch
        import sys
        request={"schema_version":1,"case_id":"unicode","operation":"messages","inputs":[],"params":{}}
        response={"schema_version":1,"case_id":"unicode","outcome":"ok","result":"中文日本語🙂","inputs_after":[],"error_kinds":[]}
        script="import sys; sys.stdout.buffer.write("+repr((json.dumps(response,ensure_ascii=False)+"\n").encode("utf-8"))+")"
        with patch("subprocess._text_encoding",return_value="cp1252"):
            self.assertEqual(driver.run([sys.executable,"-c",script],[request]),[response])


if __name__ == "__main__":
    unittest.main()
