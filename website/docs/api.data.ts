import { readFileSync } from 'node:fs';
import { methodLocales } from '../method-locales.mjs';
export default { watch: ['../api/coverage.json', '../api/rust-api.json', '../translation-manifest.json'], load() {
  const coverage = JSON.parse(readFileSync(new URL('../api/coverage.json', import.meta.url), 'utf8'));
  const inventory = JSON.parse(readFileSync(new URL('../api/rust-api.json', import.meta.url), 'utf8'));
  const signatures = new Map(inventory.map((entry: any) => [entry.id, entry.signature]));
  const manifest = JSON.parse(readFileSync(new URL('../translation-manifest.json', import.meta.url), 'utf8'));
  return coverage.map((entry: any) => ({ ...entry, signature: signatures.get(entry.id), locales: methodLocales(entry.owner, manifest.method_locale_owners) }));
} };
