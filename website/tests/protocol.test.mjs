import test from "node:test";
import assert from "node:assert/strict";
import { sourceValid, eventValid, LIMITS } from "../src/runner/protocol.js";
test("source limit counts UTF-8 bytes and rejects non-string payloads", () => {
  assert.equal(sourceValid("x".repeat(LIMITS.sourceBytes)), true);
  assert.equal(sourceValid("x".repeat(LIMITS.sourceBytes + 1)), false);
  assert.equal(sourceValid("😀".repeat(16385)), false);
  assert.equal(sourceValid({ source: "print(1)" }), false);
});
test("stale, malformed and oversized output envelopes are rejected", () => {
  assert.equal(eventValid({ type: "output", runId: 2, text: "ok" }, 2), true);
  for (const data of [
    { type: "output", runId: 1, text: "late" },
    { type: "output", runId: 2, text: {} },
    { type: "output", runId: 2, text: "x".repeat(LIMITS.outputBytes + 1) },
    { type: "error", runId: 2, category: "runtime", message: {} },
    { type: "unrecognized", runId: 2 },
  ])
    assert.equal(eventValid(data, 2), false);
});
