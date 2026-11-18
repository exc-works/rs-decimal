import {readFile} from 'node:fs/promises';
import test from 'node:test';
import assert from 'node:assert/strict';
import init,* as raw from '../public/wasm/decimal.js';
import {createBridge} from '../src/bridge/index.js';
await init({module_or_path:await readFile(new URL('../public/wasm/decimal_bg.wasm',import.meta.url))});
const bridge=createBridge(raw),{Decimal:D,NullDecimal:N,snapshot}=bridge;
const library=e=>e.category==='library'&&e.kinds.includes('Unmarshal');
test('YAML keeps exact digits, tags, quoting and first-document resolution',()=>{
 const d=D.new(9n);
 for(const [source,want] of [['012','10'],['"012"','12'],['!!int "012"','10'],['!!str 012','12'],['!int 012','12'],['0x10','16'],['1_000','1000'],['1_2.3_4e+2','1234'],['.5','0.5'],['1.','1'],['9007199254740993.0','9007199254740993'],['---\n1.23\n---\n1e100000','1.23']]){
  d.unmarshal_yaml(source);assert.equal(d.string(),want,source);
 }
 d.unmarshal_yaml('&value 1.2300 # exact');assert.equal(d.string_with_trailing_zeros(),'1.2300');assert.equal(d.marshal_yaml(),'1.23');d.free();
});
test('YAML null and errors preserve or clear complete state as Rust specifies',()=>{
 const d=D.new_from_string('12.30'),n=N.new(D.new_from_string('12.30'));n.valid=false;const before=snapshot(n);
 for(const source of ['true','".5"','"0x10"','"1_000"','[1,2]','|\n  1.2\n','---\n1\n---\n[2']){assert.throws(()=>n.unmarshal_yaml(source),library);assert.deepEqual(snapshot(n),before);}
 const saved=snapshot(d);for(const source of ['null','~','','!!null ignored']){d.unmarshal_yaml(source);assert.deepEqual(snapshot(d),saved);}
 n.unmarshal_yaml('2.500');assert.equal(n.valid,true);n.unmarshal_yaml('~');assert.deepEqual(snapshot(n),{valid:false,decimal:{coefficient:'0',scale:'0',initialized:false}});assert.equal(n.marshal_yaml(),null);
 const nil=N.new(D.default());assert.equal(nil.marshal_yaml(),'0');assert.equal(snapshot(nil).decimal.initialized,false);nil.free();n.free();d.free();
});
test('XML combines direct text, entities and CDATA while ignoring child text and later elements',()=>{
 const d=D.default();
 for(const source of ['<v a="&lt;">1.2300</v>','<v>&#49;.2300</v>','<v><![CDATA[1.2300]]></v>','<v>1.<child>ignored</child>2300</v>','<v>1.2300</v><v>1e100000</v>']){d.unmarshal_xml(source);assert.equal(d.string_with_trailing_zeros(),'1.2300');}
 assert.equal(d.marshal_xml(),'1.23');assert.equal(d.marshal_xml_attr(),'1.23');d.unmarshal_xml_attr(' 9007199254740993.2500 ');assert.equal(d.string_with_trailing_zeros(),'9007199254740993.2500');assert.throws(()=>d.unmarshal_xml_attr('&#49;'),library);d.unmarshal_xml('<v/>');assert.equal(d.is_nil(),true);assert.equal(d.marshal_xml(),'');d.free();
});
test('XML validates syntax atomically and distinguishes absent attributes from empty content',()=>{
 const n=N.new(D.new_from_string('12.30'));n.valid=false;const before=snapshot(n);
 for(const source of ['<v>bad</v>','<v>','<v>&unknown;</v>','<v>&#0;</v>','<v a="&bad;">1</v>','<1>1</1>','<v>1</x>','<v>1<c>]]></c></v>']){assert.throws(()=>n.unmarshal_xml(source),library);assert.deepEqual(snapshot(n),before);}
 assert.equal(n.marshal_xml(),'');assert.equal(n.marshal_xml_attr(),null);n.unmarshal_xml('<v>2.500</v>');assert.equal(n.valid,true);n.unmarshal_xml_attr(' \n ');assert.deepEqual(snapshot(n),{valid:false,decimal:{coefficient:'0',scale:'0',initialized:false}});
 const nil=N.new(D.default());assert.equal(nil.marshal_xml(),'0');assert.equal(nil.marshal_xml_attr(),'0');assert.equal(snapshot(nil).decimal.initialized,false);nil.free();n.free();
});
test('decoded escapes, YAML normalization and radix integers cannot bypass growth checks',()=>{
 const d=D.new_from_string('12.30'),before=snapshot(d);
 for(const source of ['"\\u0031e100000"',"'1e-100000'",'!!float 1.0e1_000_000','!!float 1.e-1000','!!int 0x'+'f'.repeat(4097),'1'.repeat(8193)]){assert.throws(()=>d.unmarshal_yaml(source),{category:'resource'},source.slice(0,40));assert.deepEqual(snapshot(d),before);}
 for(const source of ['<d>&#49;e100000</d>','<d>1<![CDATA[e100000]]></d>','<d>'+'1'.repeat(8193)+'</d>']){assert.throws(()=>d.unmarshal_xml(source),{category:'resource'});assert.deepEqual(snapshot(d),before);}
 assert.throws(()=>d.unmarshal_xml_attr('1e100000'),{category:'resource'});
 const direct=raw.Decimal.new_from_string('12.30');assert.throws(()=>direct.unmarshal_yaml('"\\u0031e100000"'),{category:'resource'});assert.throws(()=>direct.unmarshal_xml('<d>&#49;e100000</d>'),{category:'resource'});assert.equal(direct.string_with_trailing_zeros(),'12.30');direct.free();d.free();
});
test('document methods reject nonstrings and moved handles and report enabled features',()=>{
 const d=D.new(1n);assert.throws(()=>d.unmarshal_yaml(1),{category:'parameter'});assert.throws(()=>d.unmarshal_xml({}),{category:'parameter'});d.into_coefficient();assert.throws(()=>d.unmarshal_yaml('1'),{category:'moved'});assert.ok(bridge.metadata().features.includes('yaml'));assert.ok(bridge.metadata().features.includes('xml'));
});
