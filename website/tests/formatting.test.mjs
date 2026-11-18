import {readFile} from 'node:fs/promises';
import test from 'node:test';
import assert from 'node:assert/strict';
import init,* as raw from '../public/wasm/decimal.js';
import {createBridge} from '../src/bridge/index.js';
await init({module_or_path:await readFile(new URL('../public/wasm/decimal_bg.wasm',import.meta.url))});
const {Decimal:D,ByteBuffer:B,GoFormat:F}=createBridge(raw);
const kind=k=>e=>e.category==='library'&&e.kinds.includes(k);
const decode=v=>new TextDecoder().decode(v.to_uint8_array());
test('fixed text and separators preserve stored precision and Unicode scalars',()=>{
 const d=D.new_from_string('-12345.6700');assert.equal(d.string(),'-12345.67');assert.equal(d.string_with_trailing_zeros(),'-12345.6700');assert.equal(d.format_with_separators(',', '.'),'-12,345.6700');assert.equal(d.format_with_separators('💠', ','),'-12💠345,6700');assert.equal(d.format_with_separators('\0','\0'),'-12345.6700');
 for(const c of ['',',,','\ud800','\udfff','e\u0301'])assert.throws(()=>d.format_with_separators(c,'.'),{category:'parameter'});
 const rawValue=raw.Decimal.new_i64_text('1');assert.throws(()=>rawValue.format_with_separators('xx','.'),{category:'parameter'});rawValue.free();d.free();
});
test('Go verbs and flags follow core precedence and half-even',()=>{
 const d=D.new_from_string('12.50');
 for(const [f,want] of [[{},'12.5'],[{verb:'s'},'12.5'],[{verb:'q'},'"12.5"'],[{verb:'f',precision:0},'12'],[{verb:'e',precision:2},'1.25e+01'],[{verb:'E',precision:2},'1.25E+01'],[{verb:'g',precision:2},'12'],[{verb:'G',precision:2,alternate:true},'12.'],[{verb:'f',precision:1,width:8,plus:true,zero:true},'+00012.5'],[{verb:'f',precision:1,width:8,left:true,zero:true},'12.5    '],[{verb:'f',precision:1,plus:true,space:true},'+12.5'],[{verb:'f',precision:1,space:true},' 12.5'],[{verb:'d'},'%!d(decimal.Decimal=12.5)'],[{verb:'z'},'%!z(decimal.Decimal=12.5)']])assert.equal(d.format_go(f),want,JSON.stringify(f));
 const binary=D.new_from_string('1.25');assert.equal(binary.format_go({verb:'b'}),'1111101p-2');binary.free();
 const zero=D.new(0n);assert.equal(zero.format_go({verb:'d',precision:0}),'');zero.free();d.free();
});
test('format defaults are independent, strict and resource bounded in the raw ABI',()=>{
 const a=F.default(),b=F.default();a.plus=true;assert.equal(b.plus,false);assert.deepEqual(b,{verb:'v',width:null,precision:null,left:false,plus:false,space:false,zero:false,alternate:false});
 const d=D.new(1n);
 for(const bad of [null,[],{width:NaN},{width:-1},{precision:.5},{left:1},{verb:'ff'},{verb:'\ud800'},{surprise:true},{precision:4294967296}])assert.throws(()=>d.format_go(bad),{category:'parameter'});
 assert.throws(()=>d.format_go({width:65537}),{category:'resource'});assert.throws(()=>d.format_go({precision:1001}),{category:'resource'});assert.throws(()=>d.format_go({verb:'f',precision:2147483648}),kind('InvalidPrecision'));
 assert.equal(d.format_go({width:65536}).length,65536);
 const rawValue=raw.Decimal.new_i64_text('1');assert.throws(()=>rawValue.format_go('{"precision":1000000}'),{category:'resource'});assert.throws(()=>rawValue.format_go('{"width":1.5}'),{category:'parameter'});assert.throws(()=>rawValue.format_go('{"plus":1}'),{category:'parameter'});rawValue.free();d.free();
});
test('append mutates the existing buffer without consuming inputs or exposing byte views',()=>{
 const d=D.new_from_string('12.3400'),seed=new TextEncoder().encode('value='),dst=B.new(seed);seed[0]=0;
 assert.equal(d.append_text(dst),undefined);assert.equal(decode(dst),'value=12.34');assert.equal(d.append_text_with_trailing_zeros(dst),undefined);assert.equal(decode(dst),'value=12.3412.3400');
 const copy=dst.to_uint8_array();copy.fill(0);assert.equal(decode(dst),'value=12.3412.3400');assert.equal(d.string(),'12.34');d.free();dst.free();
});
test('append failure preserves receiver and destination, including near-limit buffers',()=>{
 const d=D.new_from_string('12.3400');
 for(const method of ['append_text','append_text_with_trailing_zeros']){const dst=B.new(new Uint8Array(65530).fill(65)),before=dst.to_uint8_array();assert.throws(()=>d[method](dst),{category:'resource'});assert.deepEqual(dst.to_uint8_array(),before);dst.free();}
 const valid=B.new(new Uint8Array(65520));d.append_text_with_trailing_zeros(valid);assert.equal(valid.to_uint8_array().length,65527);valid.free();
 assert.throws(()=>d.append_text(new Uint8Array()),{category:'parameter'});const foreign=createBridge(raw).ByteBuffer.new(new Uint8Array());assert.throws(()=>d.append_text(foreign),{category:'parameter'});foreign.free();
 const moved=B.new(new Uint8Array());moved.free();assert.throws(()=>d.append_text(moved),{category:'moved'});assert.equal(d.string(),'12.34');d.free();
 const rawValue=raw.Decimal.new_i64_text('1'),rawBuffer=raw.ByteBuffer.new(new Uint8Array(65536));assert.throws(()=>rawValue.append_text(rawBuffer),{category:'resource'});assert.equal(rawBuffer.to_uint8_array().length,65536);rawValue.free();rawBuffer.free();
});
