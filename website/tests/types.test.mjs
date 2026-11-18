import { readFile } from 'node:fs/promises';
import test from 'node:test';
import assert from 'node:assert/strict';
import init, * as raw from '../public/wasm/decimal.js';
import { createBridge } from '../src/bridge/index.js';
await init({module_or_path:await readFile(new URL('../public/wasm/decimal_bg.wasm',import.meta.url))});
const {Decimal,NullDecimal,snapshot,COMPATIBILITY_REVISION}=createBridge(raw);
test('validity and initialized state remain independent through field adapters',()=>{
  const value=NullDecimal.new(Decimal.default());
  assert.deepEqual(snapshot(value),{valid:true,decimal:{coefficient:'0',scale:'0',initialized:false}});
  value.valid=false;
  const replacement=Decimal.new_from_string('12.00');value.set_decimal(replacement);
  assert.throws(()=>replacement.string(),{category:'moved'});
  assert.deepEqual(snapshot(value),{valid:false,decimal:{coefficient:'1200',scale:'2',initialized:true}});
  const copy=value.get_decimal_clone();copy.free();
  assert.throws(()=>{value.valid=1;},{category:'parameter'});assert.equal(value.valid,false);
  value.valid=true;assert.equal(snapshot(value).decimal.coefficient,'1200');value.free();
});
test('compatibility revision is read from Rust and cannot be reassigned',()=>{
  assert.equal(COMPATIBILITY_REVISION,raw.compatibility_revision());
  const bridge=createBridge(raw);
  assert.throws(()=>{bridge.COMPATIBILITY_REVISION='changed';},TypeError);
});
