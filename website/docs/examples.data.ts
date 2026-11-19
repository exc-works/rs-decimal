import { readFileSync, readdirSync } from 'node:fs';
import { methodInputs, rustFields } from '../scripts/method-inputs.mjs';

const directory = new URL('../examples/', import.meta.url);
export default {
  watch: ['../examples/*.json'],
  load() {
    const files = readdirSync(directory).filter(name => name.endsWith('.json') && name !== 'index.json');
    return Object.fromEntries(files.map(name => {
      const example = JSON.parse(readFileSync(new URL(name, directory), 'utf8'));
      const inputs = methodInputs(example.javascript);
      return [example.id, { ...example, inputs, rustInputs: rustFields(example.javascript, example.rust, inputs) }];
    }));
  },
};
