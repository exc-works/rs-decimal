import test from "node:test";
import assert from "node:assert/strict";
import {
  encodeDraft,
  decodeDraft,
  decodeResult,
} from "../src/editor/draft.mjs";
test("drafts preserve source exactly, without evaluating it", () => {
  const source = 'throw new Error("never execute during restore");\n// 中文\n';
  assert.equal(decodeDraft(encodeDraft(source)), source);
  for (const text of [
    "{}",
    '{"schema":2,"source":""}',
    JSON.stringify({ schema: 1, source: "😀".repeat(16385) }),
  ])
    assert.throws(() => decodeDraft(text));
  assert.throws(() => encodeDraft("x".repeat(65537)));
});
test("restored outputs are bounded strings, never HTML nodes", () => {
  const output = [
    '"<img src=x onerror=alert(1)>"',
    '{"coefficient":"9007199254740993"}',
  ];
  assert.deepEqual(
    decodeResult(JSON.stringify({ schema: 1, output })).output,
    output,
  );
  for (const invalid of [
    { schema: 1, output: [{}] },
    { schema: 1, output: Array(101).fill("x") },
    { schema: 1, output: ["x".repeat(262145)] },
  ])
    assert.throws(() => decodeResult(JSON.stringify(invalid)));
});

import { createEditorLoader } from "../src/editor/load-once.mjs";
test("concurrent mounts wait for the same locale initialization", async () => {
  let completeLocale, imports = 0, reloads = 0;
  const locale = new Promise(resolve => { completeLocale = resolve; });
  const editor = {};
  const load = createEditorLoader({ ja: () => locale }, () => { imports++; return editor; }, () => reloads++);
  const first = load("ja"), second = load("ja");
  assert.equal(first, second);
  await Promise.resolve();
  assert.equal(imports, 0);
  completeLocale();
  assert.deepEqual(await Promise.all([first, second]), [editor, editor]);
  assert.equal(imports, 1);
  assert.equal(await load("fr"), undefined);
  assert.equal(reloads, 1);
});
test("failed locale initialization never imports an untranslated editor", async () => {
  let imports = 0;
  const load = createEditorLoader({ ja: () => Promise.reject(Error("offline")) }, () => imports++, () => {});
  await assert.rejects(load("ja"), /offline/);
  await assert.rejects(load("ja"), /offline/);
  assert.equal(imports, 0);
});
