import {readFile} from 'node:fs/promises';
import test from 'node:test';
import assert from 'node:assert/strict';
import init,* as raw from '../public/wasm/decimal.js';
import {createBridge} from '../src/bridge/index.js';
await init({module_or_path:await readFile(new URL('../public/wasm/decimal_bg.wasm',import.meta.url))});
const {Decimal:D,NullDecimal:N,snapshot,metadata}=createBridge(raw),enc=new TextEncoder(),dec=new TextDecoder();
const bytes=s=>enc.encode(s),kind=k=>e=>e.category==='library'&&e.kinds.includes(k);
test('text and parameter decoding mutate only on success and normalize parsed zero',()=>{
 const d=D.new_from_string('9.00');assert.equal(d.unmarshal_text(bytes(' 12.3400 ')),undefined);assert.equal(d.string_with_trailing_zeros(),'12.3400');assert.equal(dec.decode(d.marshal_text()),'12.34');
 d.unmarshal_param('-2.50');assert.equal(d.string_with_trailing_zeros(),'-2.50');const before=snapshot(d);
 for(const bad of [bytes('invalid'),new Uint8Array([255]),bytes(''),bytes('null')]){assert.throws(()=>d.unmarshal_text(bad),kind('Unmarshal'));assert.deepEqual(snapshot(d),before);}
 assert.throws(()=>d.unmarshal_param('1e100000'),{category:'resource'});assert.deepEqual(snapshot(d),before);
 d.unmarshal_param('0.000');assert.equal(d.scale(),0);assert.equal(d.is_nil(),false);d.free();
});
test('JSON consumes exact original numeric tokens and decoded string text',()=>{
 const d=D.default();d.unmarshal_json(bytes('9007199254740993.2500'));assert.equal(d.coefficient(),90071992547409932500n);assert.equal(d.scale(),4);assert.equal(dec.decode(d.marshal_json()),'"9007199254740993.25"');
 d.unmarshal_json(bytes('"\\u0031.2300"'));assert.equal(d.string_with_trailing_zeros(),'1.2300');const before=snapshot(d);
 d.unmarshal_json(bytes(' \t null\r\n'));assert.deepEqual(snapshot(d),before);
 for(const bad of ['+1','01','1.','[1]','true','{}','"bad"','"\\ud800"','\u00a0null']){assert.throws(()=>d.unmarshal_json(bytes(bad)),kind('Unmarshal'));assert.deepEqual(snapshot(d),before);}
 assert.throws(()=>d.unmarshal_json(new Uint8Array([34,255,34])),kind('Unmarshal'));assert.deepEqual(snapshot(d),before);d.free();
});
test('Decimal initialization and nullable validity produce distinct marshaled values',()=>{
 const empty=D.default(),zero=D.new(0n);assert.equal(dec.decode(empty.marshal_json()),'null');assert.equal(dec.decode(zero.marshal_json()),'"0"');assert.equal(dec.decode(empty.marshal_text()),'0');
 empty.unmarshal_json(bytes('null'));assert.equal(empty.is_nil(),true);
 const n=N.new(D.default());assert.equal(n.valid,true);assert.equal(n.string(),'0');assert.equal(dec.decode(n.marshal_json()),'"0"');n.valid=false;assert.equal(n.string(),'null');assert.equal(n.marshal_text().length,0);assert.equal(dec.decode(n.marshal_json()),'null');
 for(const v of [empty,zero,n])v.free();
});
test('nullable text and JSON clear differently and failures preserve both fields',()=>{
 const n=N.new(D.new_from_string('7.00'));n.valid=false;const before=snapshot(n);
 for(const method of ['unmarshal_text','unmarshal_json']){assert.throws(()=>n[method](bytes('bad')),kind('Unmarshal'));assert.deepEqual(snapshot(n),before);}
 assert.throws(()=>n.unmarshal_param('bad'),kind('Unmarshal'));assert.deepEqual(snapshot(n),before);
 n.unmarshal_json(bytes('"2.500"'));assert.equal(n.valid,true);let inner=n.get_decimal_clone();assert.equal(inner.string_with_trailing_zeros(),'2.500');inner.free();
 n.unmarshal_json(bytes('null'));assert.deepEqual(snapshot(n),{valid:false,decimal:{coefficient:'0',scale:'0',initialized:false}});
 n.unmarshal_text(bytes('4.00'));assert.equal(n.valid,true);n.unmarshal_text(new Uint8Array());assert.equal(n.valid,false);
 n.unmarshal_param('5');assert.equal(n.valid,true);n.unmarshal_param('');assert.equal(n.valid,false);
 n.unmarshal_param('6');const populated=snapshot(n);assert.throws(()=>n.unmarshal_json(new Uint8Array()),kind('Unmarshal'));assert.deepEqual(snapshot(n),populated);n.free();
});
test('decoded JSON exponent and byte budgets cannot bypass Rust preflight',()=>{
 const d=D.new_from_string('9.00'),before=snapshot(d);
 for(const input of ['1e100000','"\\u0031e100000"','"'+'9'.repeat(4097)+'"']){assert.throws(()=>d.unmarshal_json(bytes(input)),{category:'resource'});assert.deepEqual(snapshot(d),before);}
 for(const method of ['unmarshal_text','unmarshal_json']){assert.throws(()=>d[method](new Uint8Array(8193)),{category:'resource'});assert.throws(()=>d[method]('1'),{category:'parameter'});assert.deepEqual(snapshot(d),before);}
 assert.throws(()=>d.unmarshal_param(1),{category:'parameter'});assert.throws(()=>d.unmarshal_param('1'.repeat(8193)),{category:'resource'});
 const rawValue=raw.Decimal.new_from_string('9.00');assert.throws(()=>rawValue.unmarshal_json(bytes('"\\u0031e100000"')),{category:'resource'});assert.equal(rawValue.string_with_trailing_zeros(),'9.00');assert.throws(()=>rawValue.unmarshal_text(new Uint8Array(8193)),{category:'resource'});rawValue.free();d.free();
});
test('marshaled bytes are independent copies and moved wrappers are rejected',()=>{
 const d=D.new(12n),out=d.marshal_text();out.fill(0);assert.equal(dec.decode(d.marshal_text()),'12');d.into_coefficient();assert.throws(()=>d.marshal_text(),{category:'moved'});assert.throws(()=>d.unmarshal_param('1'),{category:'moved'});
});

test('metadata reports the enabled JSON feature',()=>{assert.ok(metadata().features.includes("json"));});
