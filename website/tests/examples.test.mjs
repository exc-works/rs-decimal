import {readFile,readdir} from 'node:fs/promises';
import test from 'node:test';
import assert from 'node:assert/strict';
import init,* as raw from '../public/wasm/decimal.js';
import {createBridge} from '../src/bridge/index.js';
await init({module_or_path:await readFile(new URL('../public/wasm/decimal_bg.wasm',import.meta.url))});
const bridge=createBridge(raw);
const ids=JSON.parse(await readFile(new URL('../examples/index.json',import.meta.url),'utf8'));
test('example index includes every fixture once', async () => {
  const files=(await readdir(new URL('../examples/',import.meta.url))).filter(x=>x.endsWith('.json')&&x!=='index.json').map(x=>x.slice(0,-5)).sort();
  assert.deepEqual([...ids].sort(),files);
});
for(const id of ids)test(`paired documentation example: ${id}`,async()=>{
  const example=JSON.parse(await readFile(new URL(`../examples/${id}.json`,import.meta.url),'utf8'));
  const value=new Function('Decimal',example.javascript+'\nreturn value;')(bridge.Decimal);
  assert.deepEqual(bridge.snapshot(value),example.expected);value.free();
});
