import { readFileSync, readdirSync } from 'node:fs';
import { methodInputs } from '../scripts/method-inputs.mjs';
const directory=new URL('../examples/',import.meta.url);
export default {watch:['../examples/*.json'],load(){
  return Object.fromEntries(readdirSync(directory).filter(name=>name.endsWith('.json')&&name!=='index.json').map(name=>{const value=JSON.parse(readFileSync(new URL(name,directory),'utf8'));return [value.id,{...value,inputs:methodInputs(value.javascript)}];}));
}};
