import { readFile } from 'node:fs/promises';
import test from 'node:test';
import assert from 'node:assert/strict';
import init, * as raw from '../public/wasm/decimal.js';
import { createBridge } from '../src/bridge/index.js';
const bytes=await readFile(new URL('../public/wasm/decimal_bg.wasm',import.meta.url));
await init({module_or_path:bytes});
const bridge=createBridge(raw), {Decimal,snapshot}=bridge;
test('constant helpers create independent initialized values',()=>{
  for(const [name,integer] of [['zero',0n],['one',1n],['ten',10n],['hundred',100n]]){
    const a=bridge[name](),b=bridge[name]();a.free();
    assert.equal(b.coefficient(),integer);assert.equal(b.scale(),0);assert.equal(b.is_nil(),false);b.free();
  }
});
test('free min/max preserve tie representations and between keeps inclusive ordered bounds',()=>{
  const a=Decimal.new_from_string('12.00'),b=Decimal.new_from_string('12.0'),low=bridge.zero();
  for(const name of ['min','max']){const value=bridge[name](a,b);assert.deepEqual(snapshot(value),snapshot(a));value.free();}
  assert.equal(bridge.between(a,low,b),true);assert.equal(bridge.between(low,low,b),true);assert.equal(bridge.between(a,a,low),false);
  assert.equal(a.string_with_trailing_zeros(),'12.00');a.free();b.free();low.free();
  assert.throws(()=>bridge.between(a,a,a),{category:'moved'});
});
test('must parsing preserves valid state and preflight categories',()=>{
  const value=Decimal.must_from_string('12.00');assert.equal(value.string_with_trailing_zeros(),'12.00');value.free();
  assert.throws(()=>Decimal.must_from_string(12),{category:'parameter'});
  assert.throws(()=>Decimal.must_from_string('1e100000'),{category:'resource'});
  assert.throws(()=>raw.Decimal.must_from_string('1e100000'),{category:'resource'});
  assert.throws(()=>Decimal.new_from_string('invalid'),e=>e.category==='library'&&e.kinds.includes('InvalidFormat'));
});
test('invalid must parsing traps in a dedicated Wasm instance',async()=>{
  // A panic must not be translated into new_from_string's recoverable error.
  const isolated=await import('../public/wasm/decimal.js?panic-test');
  await isolated.default({module_or_path:bytes});
  const fresh=createBridge(isolated);
  assert.throws(()=>fresh.Decimal.must_from_string('invalid'),WebAssembly.RuntimeError);
});
