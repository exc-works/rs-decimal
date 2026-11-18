import { readFile } from 'node:fs/promises';
import assert from 'node:assert/strict';
import { test } from 'node:test';
import { performance } from 'node:perf_hooks';
import init, * as raw from '../public/wasm/decimal.js';
import { createBridge } from '../src/bridge/index.js';
const bytes = await readFile(new URL('../public/wasm/decimal_bg.wasm', import.meta.url));
const exports = await init({module_or_path:bytes});
const { Decimal, ByteBuffer, snapshot } = createBridge(raw);
const resource = {category:'resource'};

test('Wasm itself rejects oversized literals and expanded exponents', () => {
  for (const value of ['1e100000','1e-100000','0e-100000','9'.repeat(4097)]) {
    assert.throws(() => raw.Decimal.new_from_string(value), resource);
    assert.throws(() => Decimal.new_from_string(value), resource);
  }
  assert.throws(() => Decimal.new_from_string('x'.repeat(8193)), resource);
  assert.throws(() => Decimal.new_from_string('invalid'), {category:'library'});
  const start = performance.now();
  const boundary = Decimal.new_from_string('9'.repeat(4096));
  assert.equal(boundary.coefficient().toString().length,4096);
  const scaled = Decimal.new_from_string('1e-1000');
  assert.equal(scaled.scale(),1000);
  assert.equal(scaled.string_with_trailing_zeros().length,1002);
  console.log(`boundary parsing/snapshot checks: ${(performance.now()-start).toFixed(2)} ms`);
  boundary.free(); scaled.free();
});

test('chained products and rescaling stay bounded and preserve borrowed inputs', () => {
  const tiny=Decimal.new_from_string('1e-600'); const before=snapshot(tiny);
  assert.throws(() => tiny.mul_exact(tiny), resource);
  assert.deepEqual(snapshot(tiny),before);
  assert.throws(() => tiny.rescale(2147483647,5), resource);
  assert.deepEqual(snapshot(tiny),before);
  const moved=tiny.clone();
  assert.throws(() => moved.into_rescaled(1001,5), resource);
  assert.throws(() => moved.string(), {category:'moved'});
  const large=Decimal.new_from_string('9'.repeat(3000));
  assert.throws(() => large.mul_exact(large), resource);
  large.free(); tiny.free(); moved.free();
});

test('raw and facade buffers enforce limits and Wasm memory has a maximum', () => {
  assert.throws(() => ByteBuffer.new(new Uint8Array(65537)), resource);
  assert.throws(() => raw.ByteBuffer.new(new Uint8Array(65537)), resource);
  const value=ByteBuffer.new(new Uint8Array(65536)); assert.equal(value.to_uint8_array().length,65536); value.free();
  assert.throws(() => exports.memory.grow(1025), RangeError);
});

test('budget cannot reset through a new facade; a fresh Wasm instance resets it', async () => {
  const value=Decimal.new(1n);
  let calls=0;
  for (;calls<10001;calls++) { try { value.scale(); } catch(e) { assert.equal(e.category,'resource'); break; } }
  assert.ok(calls>0 && calls<10000);
  assert.throws(() => value.scale(),resource);
  assert.throws(() => createBridge(raw).Decimal.default(),resource);
  value.free(); // cleanup remains available after budget exhaustion
  const fresh=await import('../public/wasm/decimal.js?fresh-resource-test');
  await fresh.default({module_or_path:bytes});
  const next=createBridge(fresh).Decimal.new(2n);assert.equal(next.string(),'2');next.free();
});
