import {readFile} from 'node:fs/promises';
import test from 'node:test';
import assert from 'node:assert/strict';
import init,* as raw from '../public/wasm/decimal.js';
import {createBridge} from '../src/bridge/index.js';
await init({module_or_path:await readFile(new URL('../public/wasm/decimal_bg.wasm',import.meta.url))});
const bridge=createBridge(raw),{Decimal:D,NullDecimal:N,DecimalError:E,ErrorKind:K,ScanValue:S,snapshot}=bridge;
const library=e=>e.category==='library'&&e.kinds.includes('Unmarshal');
test('ScanValue transports exact int64, float widths and quoted decimal bytes',()=>{
 const d=D.default();d.scan(S.Int64(9223372036854775807n));assert.equal(d.coefficient(),9223372036854775807n);
 d.scan(S.Float32(1.23456789));assert.equal(d.string(),'1.2345679');d.scan(S.Float64(1.23456789));assert.equal(d.string(),'1.23456789');
 d.scan(S.Text('"9007199254740993.2500"'));assert.equal(d.string_with_trailing_zeros(),'9007199254740993.2500');
 const bytes=new TextEncoder().encode('1.2300'),scalar=S.Bytes(bytes);bytes[0]=50;d.scan(scalar);assert.equal(d.string_with_trailing_zeros(),'2.2300');assert.equal(d.value(),'2.23');assert.equal(bytes[0],50);d.free();
});
test('SQL NULL resets Decimal while Value always returns non-null decimal zero',()=>{
 const d=D.new(9n);d.scan(S.Null);assert.equal(d.is_nil(),true);assert.equal(d.value(),'0');
 const n=N.new(d);assert.equal(n.value(),'0');n.scan(S.Null);assert.equal(n.value(),null);assert.deepEqual(snapshot(n),{valid:false,decimal:{coefficient:'0',scale:'0',initialized:false}});n.scan(S.Text('3.400'));assert.equal(n.valid,true);assert.equal(n.value(),'3.4');n.free();
});
test('SQL errors preserve both fields and keep original error categories',()=>{
 const n=N.new(D.new_from_string('12.30'));n.valid=false;const before=snapshot(n);
 for(const v of [S.Unsupported,S.Float32(Infinity),S.Float32(1e100),S.Float64(NaN),S.Text('""'),S.Text('"bad"'),S.Bytes(new Uint8Array([255]))]){assert.throws(()=>n.scan(v),library);assert.deepEqual(snapshot(n),before);}
 assert.throws(()=>n.scan(S.Text('bad')),e=>library(e)&&e.kinds.includes('InvalidFormat'));
 for(const v of [S.Text('"1e100000"'),S.Bytes(new TextEncoder().encode('"1e-100000"'))]){assert.throws(()=>n.scan(v),{category:'resource'});assert.deepEqual(snapshot(n),before);}
 const direct=raw.Decimal.new_i64_text('9');assert.throws(()=>direct.scan('Text','"1e100000"',0,new Uint8Array()),{category:'resource'});assert.equal(direct.string(),'9');direct.free();n.free();
});
test('ScanValue parameters and origins are strict without consuming borrowed values',()=>{
 for(const input of [1,1n<<63n])assert.throws(()=>S.Int64(input),{category:'parameter'});
 assert.throws(()=>S.Bytes([]),{category:'parameter'});assert.throws(()=>S.Text('1'.repeat(8193)),{category:'resource'});
 const d=D.new(1n);assert.throws(()=>d.scan({kind:'Null'}),{category:'parameter'});assert.throws(()=>d.scan(createBridge(raw).ScanValue.Null),{category:'parameter'});assert.equal(d.value(),'1');d.into_coefficient();assert.throws(()=>d.scan(S.Null),{category:'moved'});
});
test('Rust error categories retain enum ordering, deduplication, membership and message',()=>{
 assert.equal(Object.keys(K).length,11);assert.throws(()=>{K.Unmarshal='other';},TypeError);
 const original=E.new(K.Unmarshal,'bad input {0}'),error=original.with_kind(K.InvalidFormat).with_kind(K.Unmarshal);
 assert.throws(()=>original.kinds(),{category:'moved'});assert.deepEqual(error.kinds(),['InvalidFormat','Unmarshal']);assert.equal(error.matches(K.InvalidFormat),true);assert.equal(error.matches(K.Overflow),false);assert.equal(error.message,'bad input {0}');
 const copy=error.kinds();copy.length=0;assert.equal(error.kinds().length,2);error.free();
});
test('error shape validation precedes moves and does not conflate bridge failures',()=>{
 const error=E.new(K.Overflow,'overflow');assert.throws(()=>error.with_kind('fake'),{category:'parameter'});assert.equal(error.matches(K.Overflow),true);
 assert.throws(()=>E.new(K.Unmarshal,'x'.repeat(8193)),{category:'resource'});assert.throws(()=>E.new('fake','bad'),{category:'parameter'});
 assert.throws(()=>D.new_from_string('bad'),e=>e.name==='DecimalError'&&e.category==='library'&&e.kinds.includes(K.InvalidFormat));error.free();
});
