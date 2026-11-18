import {readFile} from 'node:fs/promises';
import test from 'node:test';
import assert from 'node:assert/strict';
import init,* as raw from '../public/wasm/decimal.js';
import {createBridge} from '../src/bridge/index.js';
await init({module_or_path:await readFile(new URL('../public/wasm/decimal_bg.wasm',import.meta.url))});
const bridge=createBridge(raw),{Decimal:D,NullDecimal:N,config:C,snapshot}=bridge,{ConfigValue:V}=C;
const library=e=>e.category==='library'&&e.kinds.includes('Unmarshal');
test('configuration preserves full i128/u128 and original decimal tokens',()=>{
 for(const [scalar,want] of [[V.Signed(-(1n<<127n)),(-(1n<<127n)).toString()],[V.Unsigned((1n<<128n)-1n),((1n<<128n)-1n).toString()],[V.Number('9007199254740993.2500'),'9007199254740993.2500'],[V.Text('1.2300'),'1.2300']]){const d=C.decode_decimal(scalar);assert.equal(d.string_with_trailing_zeros(),want);d.free();}
 for(const [fn,input] of [[V.Signed,1],[V.Signed,1n<<127n],[V.Unsigned,-1n],[V.Unsigned,1n<<128n]])assert.throws(()=>fn(input),{category:'parameter'});
 const bytes=new TextEncoder().encode('1.2300'),scalar=V.Bytes(bytes);bytes[0]=50;const d=C.decode_decimal(scalar);assert.equal(d.string_with_trailing_zeros(),'2.2300');d.free();
});
test('config null and exactly empty text differ from whitespace and empty number tokens',()=>{
 for(const scalar of [V.Null,V.Text(''),V.Bytes(new Uint8Array())]){assert.throws(()=>C.decode_decimal(scalar),library);const n=C.decode_nullable(scalar);assert.deepEqual(snapshot(n),{valid:false,decimal:{coefficient:'0',scale:'0',initialized:false}});n.free();}
 for(const scalar of [V.Text(' '),V.Number(''),V.Unsupported])assert.throws(()=>C.decode_nullable(scalar),library);
 assert.throws(()=>C.decode_decimal(V.Text('"1.23"')),library); // Config does not strip SQL quotes.
});
test('borrowed Decimal and Nullable config inputs clone complete state without consuming',()=>{
 const d=D.default(),a=C.decode_decimal(V.Decimal(d)),b=C.decode_nullable(V.Decimal(d));assert.equal(a.is_nil(),true);assert.deepEqual(snapshot(b),{valid:true,decimal:{coefficient:'0',scale:'0',initialized:false}});
 const n=N.new(D.new_from_string('12.30'));n.valid=false;const scalar=V.Nullable(n),before=snapshot(n),copy=C.decode_nullable(scalar);assert.deepEqual(snapshot(copy),before);assert.throws(()=>C.decode_decimal(scalar),library);assert.deepEqual(snapshot(n),before);
 n.valid=true;const present=C.decode_decimal(scalar);assert.equal(present.string_with_trailing_zeros(),'12.30');assert.equal(n.valid,true);
 const stale=V.Decimal(d);d.free();assert.throws(()=>C.decode_decimal(stale),{category:'moved'});a.free();b.free();n.free();copy.free();present.free();
});
test('config float widths, unsupported values and original errors stay in Rust',()=>{
 const d=C.decode_decimal(V.Float32(1.23456789));assert.equal(d.string(),'1.2345679');d.free();
 for(const scalar of [V.Float32(1e100),V.Float64(NaN),V.Bytes(new Uint8Array([255]))])assert.throws(()=>C.decode_decimal(scalar),library);
 for(const scalar of [V.Number('1e100000'),V.Bytes(new TextEncoder().encode('1e-100000'))])assert.throws(()=>C.decode_decimal(scalar),{category:'resource'});
 assert.throws(()=>C.decode_decimal({kind:'Null'}),{category:'parameter'});assert.throws(()=>C.decode_nullable(createBridge(raw).config.ConfigValue.Null),{category:'parameter'});
});
test('generic Serde uses strings/exact integer visitors and rejects fractional JSON numbers',()=>{
 const d=D.deserialize('"9007199254740993.2500"');assert.equal(d.string_with_trailing_zeros(),'9007199254740993.2500');assert.equal(d.serialize(),'"9007199254740993.25"');d.free();
 const integer=D.deserialize('9007199254740993');assert.equal(integer.coefficient(),9007199254740993n);integer.free();
 for(const source of ['1.25','1e2','true','{}','""'])assert.throws(()=>D.deserialize(source),{category:'serde'});
 const nil=D.deserialize('null');assert.equal(nil.is_nil(),true);assert.equal(nil.serialize(),'null');nil.free();
 const n=N.new(D.default());assert.equal(n.serialize(),'"0"');n.valid=false;assert.equal(n.serialize(),'null');n.free();
 const absent=N.deserialize('null');assert.equal(absent.valid,false);absent.free();
});
test('JSON configuration adapters preserve raw numbers and have distinct empty/null rules',()=>{
 const d=C.deserialize_json_decimal('9007199254740993.2500');assert.equal(d.string_with_trailing_zeros(),'9007199254740993.2500');d.free();
 for(const source of ['null','""','true','1 2'])assert.throws(()=>C.deserialize_json_decimal(source),{category:'serde'});
 for(const source of ['null','""']){const n=C.deserialize_json_nullable(source);assert.equal(n.valid,false);assert.equal(snapshot(n).decimal.initialized,false);n.free();}
 assert.throws(()=>C.deserialize_json_nullable('" "'),{category:'serde'});
 const n=C.deserialize_json_nullable('"1.2300"');assert.equal(n.valid,true);assert.equal(snapshot(n).decimal.scale,'4');n.free();
});
test('decoded JSON and raw ABI cannot evade config or Serde resource preflight',()=>{
 for(const fn of [D.deserialize,N.deserialize,C.deserialize_json_decimal,C.deserialize_json_nullable])for(const source of ['"\\u0031e100000"','"1e-100000"','1'.repeat(8193)])assert.throws(()=>fn(source),{category:'resource'});
 assert.throws(()=>raw.config_decode_decimal('Number','1e100000',0,new Uint8Array()),{category:'resource'});assert.throws(()=>raw.config_deserialize_json_decimal('"\\u0031e100000"'),{category:'resource'});
 assert.ok(bridge.metadata().features.includes('config'));assert.ok(bridge.metadata().features.includes('serde'));
});

test('complete JSON syntax is checked before any scalar visitor can expand a coefficient',()=>{
 const sources=['"1e100000" 0','"\\u0031e100000" []','"1e10000000" 0','"1e-100000" false'];
 for(const fn of [D.deserialize,N.deserialize,C.deserialize_json_decimal,C.deserialize_json_nullable]){
  for(const source of sources)assert.throws(()=>fn(source),{category:'serde'});
  assert.throws(()=>fn('"1e100000"'),{category:'resource'});
 }
 for(const fn of [raw.Decimal.deserialize,raw.NullDecimal.deserialize,raw.config_deserialize_json_decimal,raw.config_deserialize_json_nullable]){
  assert.throws(()=>fn('"1e10000000" 0'),{category:'serde'});
  assert.throws(()=>fn('"1e100000"'),{category:'resource'});
 }
});
