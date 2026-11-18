import { readFile } from 'node:fs/promises';
import { createHash } from 'node:crypto';
import { pathToFileURL } from 'node:url';
import { locales } from '../locales.mjs';
import { methodLocales } from '../method-locales.mjs';
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
export async function checkMethodTranslations(root = new URL('../', import.meta.url), release = false) {
  const entries = JSON.parse(await readFile(new URL('api/coverage.json', root), 'utf8'));
  const manifest = JSON.parse(await readFile(new URL('translation-manifest.json', root), 'utf8'));
  const groups = manifest.method_locale_owners ?? {};
  const owners = new Set(entries.map(entry => entry.owner));
  if (!groups || Array.isArray(groups) || typeof groups !== 'object' || Object.entries(groups).some(([locale, values]) => !expectedLocales.includes(locale) || !Array.isArray(values) || !values.length || new Set(values).size !== values.length || values.some(owner => owner !== '*' && !owners.has(owner)))) throw Error('Invalid method translation groups');
  const implemented = entries.filter(x => x.status === 'implemented');
  for (const entry of implemented) {
    const source = await readFile(new URL(`docs/en-US/api/${entry.doc_id}.md`, root), 'utf8');
    const body = source.replace(/^---\n[\s\S]*?\n---\n/, '');
    const revision = createHash('sha256').update(body).digest('hex');
    const required = methodLocales(entry.owner, groups);
    for (const locale of release ? expectedLocales : required) {
      const page = await readFile(new URL(`docs/${locale}/api/${entry.doc_id}.md`, root), 'utf8');
      const header = page.match(/^---\n([\s\S]*?)\n---\n/)?.[1].split('\n');
      if (header?.includes('translation_status: pending') || !header?.includes(`locale: ${locale}`) || !header.includes(`source_revision: ${revision}`) || (!header.includes(`api_id: ${entry.id}`) && !header.includes(`api_id: ${JSON.stringify(entry.id)}`)) || !/^# .+/m.test(page)) throw Error(`Missing/stale method translation: ${locale}/${entry.doc_id}`);
    }
  }
  return implemented.length;
}
if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  const count = await checkTranslations();
  const methods = await checkMethodTranslations(undefined, process.argv.includes('--release'));
  console.log(`${methods} implemented methods have current reference translations. ${expectedLocales.length} locales, ${count} current shared documentation units; API completeness remains a separate release gate.`);
}
