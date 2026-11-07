"""Certify float outputs using exact Fraction midpoint cells, not production rounding."""
import argparse
from fractions import Fraction
import importlib.util
import json
from pathlib import Path

spec = importlib.util.spec_from_file_location("driver", Path(__file__).with_name("run.py"))
driver = importlib.util.module_from_spec(spec)
spec.loader.exec_module(driver)


def value(bits, width):
    fraction, exponent = (23, 8) if width == 32 else (52, 11)
    bias = (1 << (exponent-1))-1
    exp, mantissa = bits >> fraction, bits & ((1 << fraction)-1)
    if exp:
        mantissa += 1 << fraction
    return Fraction(mantissa) * Fraction(2)**((exp or 1)-bias-fraction)


def certify(state, result):
    driver.validate_state(state)
    width = int(result["width"])
    fraction, exponent = (23, 8) if width == 32 else (52, 11)
    bias = (1 << (exponent-1))-1
    signed = int(result["bits"], 16)
    sign = signed >> (width-1)
    bits = signed & ((1 << (width-1))-1)
    x = Fraction(int(state["coefficient"]), 10**int(state["scale"]))
    assert sign == (x < 0), "floating sign differs (including underflow zero)"
    x = abs(x)
    infinity = ((1 << exponent)-1) << fraction
    assert bits <= infinity, "NaN cannot be a decimal conversion result"
    if bits == infinity:
        threshold = value(infinity-1, width) + Fraction(2)**(bias-fraction-1)
        assert x >= threshold and result["exact"] is False, "uncertified overflow"
        return
    y = value(bits, width)
    assert result["exact"] is (x == y), "incorrect exact flag"
    if bits == 0:
        assert x <= value(1, width)/2, "zero outside nearest-even cell"
        return
    lo = (value(bits-1, width)+y)/2
    next_value = Fraction(2)**(bias+1) if bits+1 == infinity else value(bits+1, width)
    hi = (y+next_value)/2
    assert lo <= x <= hi, "value outside exact midpoint cell"
    assert bits % 2 == 0 or (lo < x < hi), "odd significand cannot own midpoint tie"


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--rust", required=True)
    parser.add_argument("--cases", required=True)
    args = parser.parse_args()
    cases = [json.loads(line) for line in Path(args.cases).read_text(encoding="utf-8").splitlines() if line.strip()]
    cases = [c for c in cases if c["operation"] == "to_float"]
    assert cases, "empty floating oracle selection"
    for c in cases:
        c.pop("expected_outcome", None)
    responses = driver.run([args.rust], cases, 60)
    for req, response in zip(cases, responses):
        assert response["outcome"] == "ok", req["case_id"]
        assert response["result"]["width"] == req["params"]["width"], "wrong target width"
        certify(req["inputs"][0], response["result"])
    # Guard the oracle's rejection paths, including last-bit and exact-flag mutations.
    state = {"coefficient": "1", "scale": "0", "initialized": True}
    for wrong in [{"width": "32", "bits": "3f800001", "exact": False},
                  {"width": "32", "bits": "3f800000", "exact": False},
                  {"width": "32", "bits": "bf800000", "exact": True}]:
        try:
            certify(state, wrong)
        except AssertionError:
            continue
        raise AssertionError("oracle accepted mutated floating result")
    print(f"Exact midpoint oracle verified {len(cases)} floating conversions")


if __name__ == "__main__":
    main()
