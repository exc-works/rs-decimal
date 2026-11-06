"""Batch JSONL conformance driver. Unknown operations and transport failures fail closed."""
import argparse
import collections
import json
from pathlib import Path
import subprocess
import re


class ConformanceError(RuntimeError):
    pass


def validate_state(state):
    if not isinstance(state, dict) or set(state) != {"coefficient", "scale", "initialized"}:
        raise ConformanceError("incomplete decimal state")
    c, s, initialized = state["coefficient"], state["scale"], state["initialized"]
    if not isinstance(c, str) or not re.fullmatch(r"0|-?[1-9][0-9]*", c):
        raise ConformanceError("noncanonical coefficient")
    if not isinstance(s, str) or not re.fullmatch(r"0|[1-9][0-9]*", s):
        raise ConformanceError("noncanonical scale")
    if type(initialized) is not bool or (not initialized and (c != "0" or s != "0")):
        raise ConformanceError("invalid initialization state")


def run(command, requests, timeout=30):
    ids = [r["case_id"] for r in requests]
    if len(set(ids)) != len(ids) or not all(ids):
        raise ConformanceError("missing or duplicate case IDs")
    payload = "".join(json.dumps(r) + "\n" for r in requests)
    try:
        process = subprocess.run(command, input=payload, text=True, capture_output=True, timeout=timeout)
    except subprocess.TimeoutExpired as exc:
        raise ConformanceError("timeout") from exc
    if process.returncode:
        raise ConformanceError(f"process exited {process.returncode}: {process.stderr[-2000:]}")
    try:
        responses = [json.loads(line) for line in process.stdout.splitlines()]
    except (ValueError, TypeError) as exc:
        raise ConformanceError("non-protocol stdout") from exc
    if len(responses) != len(requests):
        raise ConformanceError("response count mismatch")
    for request, response in zip(requests, responses):
        if not isinstance(response, dict) or response.get("schema_version") != 1 or response.get("case_id") != request["case_id"]:
            raise ConformanceError("response identity mismatch")
    return responses


def compare(requests, left, right, expected=None):
    if not (len(requests) == len(left) == len(right)):
        raise ConformanceError("comparison count mismatch")
    expected = expected or ["ok"] * len(requests)
    if len(expected) != len(requests):
        raise ConformanceError("expected outcome count mismatch")
    counts = collections.Counter()
    for req, a, b, want in zip(requests, left, right, expected):
        for response in (a, b):
            if response.get("schema_version") != 1 or response.get("case_id") != req["case_id"]:
                raise ConformanceError("comparison identity mismatch")
            if response.get("outcome") not in ("ok", "error", "panic"):
                raise ConformanceError(f"{req['case_id']}: unsupported or invalid outcome")
            if response.get("outcome") != want:
                raise ConformanceError(f"{req['case_id']}: unexpected {response.get('outcome')}")
            if not isinstance(response.get("error_kinds"), list) or not isinstance(response.get("inputs_after"), list):
                raise ConformanceError("incomplete response state")
            if not all(isinstance(k, str) and k for k in response["error_kinds"]):
                raise ConformanceError("invalid error categories")
            if len(response["inputs_after"]) != len(req["inputs"]):
                raise ConformanceError("input state count mismatch")
            for state in response["inputs_after"]:
                validate_state(state)
            if want == "ok" and "result" not in response:
                raise ConformanceError("missing successful result")
            if response["inputs_after"] != req["inputs"]:
                raise ConformanceError("operation mutated input operands")
            if want == "ok" and req["operation"] in {"state", "parse", "must_parse", "construct", "min", "max", "clone", "abs", "neg", "rescale", "rescale_down", "floor", "ceil", "round", "truncate", "shift", "strip", "significant", "add", "sub", "safe_add", "safe_sub", "mul", "mul_down", "mul_exact", "mul_prec", "quo", "quo_down", "quo_prec", "mod", "power", "add_raw", "sub_raw", "must_non_negative", "from_float", "from_rat", "from_rat_prec", "sqrt", "root", "sqrt_prec", "root_prec", "ln", "log2", "log10", "exp", "ln_prec", "log2_prec", "log10_prec", "exp_prec"}:
                validate_state(response["result"])
            if want == "ok" and req["operation"] in {"format", "string", "trailing", "separators", "append", "encode_text", "encode_json", "encode_binary"} and not isinstance(response["result"], str):
                raise ConformanceError("invalid text result")
            if want == "ok" and req["operation"] in {"encode_yaml", "encode_xml"} and response["result"] is not None and not isinstance(response["result"], str):
                raise ConformanceError("invalid YAML scalar")
            if want == "ok" and req["operation"] == "encode_bson":
                result = response["result"]
                if not isinstance(result, dict) or set(result) != {"type", "data"} or result["type"] not in {"2", "10"} or not isinstance(result["data"], str) or not re.fullmatch(r"(?:[0-9a-f]{2})*", result["data"]):
                    raise ConformanceError("invalid BSON payload")
            if req["operation"] == "decode":
                receiver = response.get("receiver_after")
                nullable = req["params"]["nullable"] == "true"
                if not isinstance(receiver, dict) or set(receiver) != {"decimal", "valid"}:
                    raise ConformanceError("missing complete receiver state")
                validate_state(receiver["decimal"])
                if nullable and type(receiver["valid"]) is not bool or not nullable and receiver["valid"] is not None:
                    raise ConformanceError("invalid receiver validity")
                if want != "ok" and receiver != {"decimal": req["inputs"][0], "valid": req["params"]["valid"] == "true" if nullable else None}:
                    raise ConformanceError("failed decode changed receiver")
                if want == "ok" and response["result"] != receiver:
                    raise ConformanceError("decode result disagrees with receiver")
            if want == "ok" and req["operation"] == "to_float":
                result = response["result"]
                if not isinstance(result, dict) or set(result) != {"width", "bits", "exact"} or result["width"] not in ("32", "64") or type(result["exact"]) is not bool or not isinstance(result["bits"], str) or not re.fullmatch(r"[0-9a-f]{"+str(int(result["width"])//4)+r"}",result["bits"]):
                    raise ConformanceError("invalid float result")
            if want == "ok" and req["operation"] in {"to_rat", "native_int"}:
                result = response["result"]
                nums = {"numerator", "denominator"} if req["operation"] == "to_rat" else {"signed", "unsigned", "bit_len"}
                bools = set() if req["operation"] == "to_rat" else {"signed_exact", "unsigned_exact"}
                if not isinstance(result, dict) or set(result) != nums | bools or any(type(result[k]) is not bool for k in bools) or any(not isinstance(result[k], str) or not re.fullmatch(r"0|-?[1-9][0-9]*",result[k]) for k in nums):
                    raise ConformanceError("invalid rational/native result")
            if want == "ok" and req["operation"] == "msb" and (not isinstance(response["result"], str) or not re.fullmatch(r"0|[1-9][0-9]*",response["result"])):
                raise ConformanceError("invalid bit index")
            if want == "ok" and req["operation"] == "quo_rem":
                result = response["result"]
                if not isinstance(result, list) or len(result) != 2:
                    raise ConformanceError("incomplete quotient/remainder result")
                for state in result:
                    validate_state(state)
            if want == "ok" and req["operation"] == "integer_parts":
                result = response["result"]
                if not isinstance(result, dict) or set(result) != {"int_part", "integer", "fraction", "is_integer", "has_fraction"}:
                    raise ConformanceError("incomplete integer inspection")
                if any(type(result[k]) is not bool for k in ["is_integer", "has_fraction"]) or any(not isinstance(result[k], str) or not re.fullmatch(r"0|-?[1-9][0-9]*", result[k]) for k in ["int_part", "integer", "fraction"]):
                    raise ConformanceError("invalid integer inspection types")
            if want == "ok" and req["operation"] == "mode" and response["result"] not in [str(i) for i in range(7)]:
                raise ConformanceError("invalid rounding mode result")
            if want == "ok" and req["operation"] in {"inspect", "cmp"}:
                result = response["result"]
                numeric = {"sign", "precision"} if req["operation"] == "inspect" else {"cmp"}
                boolean = {"nil", "zero", "not_zero", "positive", "negative"} if req["operation"] == "inspect" else {"equal", "not_equal", "gt", "gte", "lt", "lte"}
                if not isinstance(result, dict) or set(result) != numeric | boolean:
                    raise ConformanceError("incomplete inspection/comparison result")
                if any(type(result[k]) is not bool for k in boolean) or any(not isinstance(result[k], str) or not re.fullmatch(r"0|-?[1-9][0-9]*", result[k]) for k in numeric):
                    raise ConformanceError("invalid inspection/comparison result types")
            if want == "ok" and req["operation"] == "between" and type(response["result"]) is not bool:
                raise ConformanceError("invalid boolean result")
        a, b = dict(a), dict(b)
        a["error_kinds"] = sorted(set(a["error_kinds"]))
        b["error_kinds"] = sorted(set(b["error_kinds"]))
        if a != b:
            raise ConformanceError(f"{req['case_id']}: difference\nGo: {a}\nRust: {b}")
        counts[want] += 1
    return dict(counts)


def verify_oracle(command, requests, timeout=30):
    responses = run(command, requests, timeout)
    for response in responses:
        if response.get("status") != "verified":
            raise ConformanceError(f"oracle {response['case_id']}: {response}")
    return len(responses)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--go", required=True)
    parser.add_argument("--rust", required=True)
    parser.add_argument("--cases", required=True)
    parser.add_argument("--timeout", type=float, default=30)
    args = parser.parse_args()
    cases = [json.loads(line) for line in Path(args.cases).read_text().splitlines() if line.strip()]
    if not cases:
        raise ConformanceError("empty corpus")
    expectations = [c.pop("expected_outcome", "ok") for c in cases]
    left = run([args.go], cases, args.timeout)
    right = run([args.rust], cases, args.timeout)
    try:
        counts = compare(cases, left, right, expectations)
    except ConformanceError:
        failure_dir = Path(".cache/conformance-failures")
        failure_dir.mkdir(parents=True, exist_ok=True)
        (failure_dir / "latest.json").write_text(json.dumps({"requests": cases, "go": left, "rust": right}, indent=2)+"\n")
        raise
    print(json.dumps({"cases": len(cases), "outcomes": counts}))


if __name__ == "__main__":
    main()
