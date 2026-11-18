import {readFile} from 'node:fs/promises';
import test from 'node:test';
import assert from 'node:assert/strict';
import init,* as raw from '../public/wasm/decimal.js';
import {createBridge} from '../src/bridge/index.js';
await init({module_or_path:await readFile(new URL('../public/wasm/decimal_bg.wasm',import.meta.url))});
const bridge=createBridge(raw),{Decimal:D,NullDecimal:N,snapshot}=bridge;
const kind=k=>e=>e.category==='library'&&e.kinds.includes(k);
const bytes=a=>new Uint8Array(a);
function decimal128(coefficient,exponent){const a=new Uint8Array(16),v=new DataView(a.buffer);v.setBigUint64(0,coefficient,true);v.setBigUint64(8,BigInt(6176+exponent)<<49n,true);return a;}
function bsonString(text){const content=new TextEncoder().encode(text),a=new Uint8Array(content.length+5);new DataView(a.buffer).setInt32(0,content.length+1,true);a.set(content,4);return a;}
test('binary golden bytes normalize scale and distinguish nil/empty initialized zero',()=>{
 for(const [source,want] of [['0',[0,0,0,0,2]],['1.23',[0,0,0,2,2,123]],['-1.23',[0,0,0,2,3,123]],['1.2300',[0,0,0,2,2,123]]]){
  const d=D.new_from_string(source),encoded=d.marshal_binary(),decoded=D.default();assert.deepEqual(Array.from(encoded),want);decoded.unmarshal_binary(encoded);assert.equal(decoded.equal(d),true);assert.equal(decoded.is_nil(),false);assert.equal(decoded.scale(),source==='0'?0:2);d.free();decoded.free();
 }
 const nil=D.default();assert.equal(nil.marshal_binary().length,0);nil.unmarshal_binary(bytes([]));assert.equal(nil.is_nil(),false);nil.unmarshal_binary(bytes([0,0,0,3]));assert.equal(nil.scale(),3);assert.equal(nil.coefficient(),0n);nil.free();
});
test('binary resource and core errors are atomic and enforce actual coefficient bits',()=>{
 const d=D.new_from_string('12.30'),before=snapshot(d);
 for(const b of [[0],[0,0,0],[0,0,0,1,4],[0,2,0,1,2]]){assert.throws(()=>d.unmarshal_binary(bytes(b)),kind('Unmarshal'));assert.deepEqual(snapshot(d),before);}
 for(const b of [bytes([0,0,3,233,2,1]),bytes([0,0,0,0,2,...new Uint8Array(2049).fill(255)]),new Uint8Array(65537)]){assert.throws(()=>d.unmarshal_binary(b),{category:'resource'});assert.deepEqual(snapshot(d),before);}
 const padded=bytes([0,0,0,0,2,...new Uint8Array(3000),1]);d.unmarshal_binary(padded);assert.equal(d.string(),'1');
 const rawValue=raw.Decimal.new_from_string('12.30');assert.throws(()=>rawValue.unmarshal_binary(bytes([0,0,3,233,2,1])),{category:'resource'});assert.equal(rawValue.string_with_trailing_zeros(),'12.30');rawValue.free();d.free();
});
test('BSON tuples contain type byte and scalar payload with preserved compatibility quirks',()=>{
 const d=D.new_from_string('1.2300'),[k,payload]=d.marshal_bson_value();assert.equal(k,2);assert.deepEqual(Array.from(payload),[5,0,0,0,49,46,50,51,0]);payload.fill(0);assert.equal(d.marshal_bson_value()[1][4],49);
 d.unmarshal_bson_value(19,decimal128(12300n,-4));assert.equal(d.string_with_trailing_zeros(),'1.2300');
 const integer=new Uint8Array(8);new DataView(integer.buffer).setBigInt64(0,9223372036854775807n,true);d.unmarshal_bson_value(18,integer);assert.equal(d.coefficient(),9223372036854775807n);
 const double=new Uint8Array(8);new DataView(double.buffer).setFloat64(0,1.25,true);d.unmarshal_bson_value(1,double);assert.equal(d.string(),'1.25');
 d.unmarshal_bson_value(2,bytes([3,0,0,0,49,50,120,101]));assert.equal(d.string(),'12'); // final payload byte need not be NUL; trailing data ignored
 d.unmarshal_bson_value(10,bytes([255]));assert.equal(d.is_nil(),true);assert.deepEqual(d.marshal_bson_value(),[10,new Uint8Array()]);d.free();
});
test('BSON validation and growth preflight preserve both nullable fields',()=>{
 const n=N.new(D.new_from_string('12.30'));n.valid=false;const before=snapshot(n);
 for(const [k,b] of [[8,bytes([1])],[19,new Uint8Array(15)],[2,bytes([255,255,255,255])]]){assert.throws(()=>n.unmarshal_bson_value(k,b),kind('Unmarshal'));assert.deepEqual(snapshot(n),before);}
 const nonfinite=new Uint8Array(16);new DataView(nonfinite.buffer).setBigUint64(8,0x7800000000000000n,true);assert.throws(()=>n.unmarshal_bson_value(19,nonfinite),kind('InvalidFormat'));assert.deepEqual(snapshot(n),before);
 for(const [k,b] of [[19,decimal128(1n,1001)],[19,decimal128(1n,-1001)],[2,bsonString('1e100000')]]){assert.throws(()=>n.unmarshal_bson_value(k,b),{category:'resource'});assert.deepEqual(snapshot(n),before);}
 for(const k of [-1,256,.5,'2'])assert.throws(()=>n.unmarshal_bson_value(k,new Uint8Array()),{category:'parameter'});
 n.unmarshal_bson_value(2,bsonString('4.00'));assert.equal(n.valid,true);n.unmarshal_bson_value(10,new Uint8Array());assert.deepEqual(snapshot(n),{valid:false,decimal:{coefficient:'0',scale:'0',initialized:false}});n.free();
 const rawValue=raw.Decimal.new_i64_text('1');assert.throws(()=>rawValue.unmarshal_bson_value(19,decimal128(1n,1001)),{category:'resource'});assert.throws(()=>rawValue.unmarshal_bson_value(.5,new Uint8Array()),{category:'parameter'});rawValue.free();
});
test('valid nullable uninitialized zero marshals as BSON string without changing state',()=>{
 const n=N.new(D.default()),before=snapshot(n);assert.deepEqual(n.marshal_bson_value(),[2,bytes([2,0,0,0,48,0])]);assert.deepEqual(snapshot(n),before);n.valid=false;assert.deepEqual(n.marshal_bson_value(),[10,new Uint8Array()]);n.free();
});
test('precision prefix metadata is Rust-backed and immutable',()=>{
 assert.equal(bridge.PRECISION_FIXED_SIZE,4);assert.throws(()=>{bridge.PRECISION_FIXED_SIZE=8;},TypeError);assert.ok(bridge.metadata().features.includes('bson'));
});
