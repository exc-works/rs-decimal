import {readFile,readdir} from 'node:fs/promises';
import test from 'node:test';
import assert from 'node:assert/strict';
import init,* as raw from '../public/wasm/decimal.js';
import { receiptSource, isStockSource } from '../src/editor/receipt.mjs';
import { locales } from '../locales.mjs';
import {createBridge} from '../src/bridge/index.js';
await init({module_or_path:await readFile(new URL('../public/wasm/decimal_bg.wasm',import.meta.url))});
const bridge=createBridge(raw);
const ids=JSON.parse(await readFile(new URL('../examples/index.json',import.meta.url),'utf8'));
test('example index includes every fixture once', async () => {
  const files=(await readdir(new URL('../examples/',import.meta.url))).filter(x=>x.endsWith('.json')&&x!=='index.json').map(x=>x.slice(0,-5)).sort();
  assert.deepEqual([...ids].sort(),files);
});
const coverage=JSON.parse(await readFile(new URL('../api/coverage.json',import.meta.url),'utf8'));
for(const id of ids){
  const example=JSON.parse(await readFile(new URL(`../examples/${id}.json`,import.meta.url),'utf8'));
  const nativeOnly=!example.javascript.trim();
  if(nativeOnly) assert.ok(example.api_ids.every(api=>coverage.find(row=>row.id===api)?.disposition==='rust-only'), `Missing JS for ${id}`);
  test(`documentation example: ${id}`,{skip:nativeOnly?'Rust-only syntax; executed by native harness':false},()=>{
    const value=new Function(...Object.keys(bridge),example.javascript+'\nreturn value;')(...Object.values(bridge));
    assert.deepEqual(value instanceof bridge.Decimal || value instanceof bridge.NullDecimal ? bridge.snapshot(value) : value,example.expected);value.free?.();
  });
}

test('every starter receipt computes the displayed totals when the quantity changes', () => {
  for (const lang of Object.keys(locales)) {
    const starter = receiptSource(lang);
    assert.ok(isStockSource(starter));
    assert.ok(isStockSource(starter.replaceAll("console.log(", "show(")));
    for (const [quantity, expected] of [[2, ['9.00', '1.25', '10.25', '9.75']], [3, ['13.50', '1.25', '14.75', '5.25']]]) {
      const source = starter.replace('Decimal.new(2n)', 'Decimal.new(' + quantity + 'n)');
      if (quantity === 3) assert.equal(isStockSource(source), false);
      const lines = [];
      new Function('Decimal', 'console', source)(bridge.Decimal, {log: line => lines.push(line)});
      assert.deepEqual(lines.map(line => line.split(': ').at(-1)), expected);
    }
  }
});
