import { readFile } from 'node:fs/promises';
import { createHash } from 'node:crypto';
import { pathToFileURL } from 'node:url';
import { locales } from '../locales.mjs';
export const expectedLocales = ['zh-Hans','zh-Hant','en-US','en-GB','fr','ko','ja','de'];
export async function checkTranslations(root = new URL('../', import.meta.url), labels = locales) {
  const manifest = JSON.parse(await readFile(new URL('translation-manifest.json', root), 'utf8'));
  if (manifest.schema_version !== 1 || manifest.source_locale !== 'en-US' || !Array.isArray(manifest.units) || !manifest.units.length || new Set(manifest.units).size !== manifest.units.length || manifest.units.some(x => typeof x !== 'string' || !/^[a-z0-9-]+(?:\/[a-z0-9-]+)*$/.test(x))) throw Error('Invalid translation manifest');
  if (JSON.stringify(Object.keys(labels).sort()) !== JSON.stringify([...expectedLocales].sort())) throw Error('Locale set mismatch');
  const keys = Object.keys(labels['en-US']).sort();
  for (const [locale, values] of Object.entries(labels)) {
    if (JSON.stringify(Object.keys(values).sort()) !== JSON.stringify(keys) || Object.values(values).some(x => typeof x !== 'string' || !x.trim())) throw Error(`Missing interface labels: ${locale}`);
  }
  for (const unit of manifest.units) {
    const source = await readFile(new URL(`docs/en-US/${unit}.md`, root), 'utf8');
    const body = source.replace(/^---\n[\s\S]*?\n---\n/, '');
    const revision = createHash('sha256').update(body).digest('hex');
    for (const locale of expectedLocales) {
      const page = await readFile(new URL(`docs/${locale}/${unit}.md`, root), 'utf8');
      if (!page.startsWith(`---\nlocale: ${locale}\nsource_revision: ${revision}\n---\n`)) throw Error(`Missing/stale translation: ${locale}/${unit}`);
      if (!/^# .+/m.test(page)) throw Error(`Empty page: ${locale}/${unit}`);
    }
  }
  return manifest.units.length;
}
if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  const count = await checkTranslations();
  console.log(`${expectedLocales.length} locales, ${count} current shared documentation units; API completeness remains a separate release gate.`);
}
