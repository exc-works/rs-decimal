import { readFileSync } from 'node:fs';
export default { watch: ['../api/coverage.json', '../api/rust-api.json'], load() {
  const coverage = JSON.parse(readFileSync(new URL('../api/coverage.json', import.meta.url), 'utf8'));
  const inventory = JSON.parse(readFileSync(new URL('../api/rust-api.json', import.meta.url), 'utf8'));
  const signatures = new Map(inventory.map((entry: any) => [entry.id, entry.signature]));
  return coverage.map((entry: any) => ({ ...entry, signature: signatures.get(entry.id) }));
} };
