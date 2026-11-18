import test from 'node:test';
import assert from 'node:assert/strict';
import { mkdtemp, mkdir, writeFile, rm } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { pathToFileURL } from 'node:url';
import { createHash } from 'node:crypto';
import { checkTranslations, checkMethodTranslations, expectedLocales } from '../scripts/check-i18n.mjs';
import { locales } from '../locales.mjs';

test('all published shared pages have current translations and interface labels', async () => {
  assert.equal(await checkTranslations(), 5);
});
test('source edits, missing locales and incomplete labels fail closed', async () => {
  const dir = await mkdtemp(join(tmpdir(), 'decimal-i18n-'));
  const root = pathToFileURL(dir + '/');
  const source = '# Introduction\n\nExact input.\n';
  const revision = createHash('sha256').update(source).digest('hex');
  const page = locale => `---\nlocale: ${locale}\nsource_revision: ${revision}\n---\n${source}`;
  try {
    await writeFile(new URL('translation-manifest.json', root), JSON.stringify({schema_version:1,source_locale:'en-US',units:['index']}));
    for (const locale of expectedLocales) {
      await mkdir(new URL(`docs/${locale}/`, root), {recursive:true});
      await writeFile(new URL(`docs/${locale}/index.md`, root), page(locale));
    }
    assert.equal(await checkTranslations(root), 1);
    await writeFile(new URL('docs/en-US/index.md', root), page('en-US') + 'Changed meaning.\n');
    await assert.rejects(checkTranslations(root), /stale translation/);
    await writeFile(new URL('docs/en-US/index.md', root), page('en-US'));
    await rm(new URL('docs/ja/index.md', root));
    await assert.rejects(checkTranslations(root), /ENOENT/);
    await writeFile(new URL('docs/ja/index.md', root), page('ja'));
    const broken = structuredClone(locales); delete broken.fr.search;
    await assert.rejects(checkTranslations(root, broken), /Missing interface labels: fr/);
  } finally { await rm(dir, {recursive:true,force:true}); }
});

test('method revisions and API identities cannot silently drift', async () => {
  assert.equal(await checkMethodTranslations(), 355);
  const dir = await mkdtemp(join(tmpdir(), 'decimal-method-i18n-'));
  const root = pathToFileURL(dir + '/');
  const body = '# Decimal::new\n\nCreates an integer.\n';
  const hash = createHash('sha256').update(body).digest('hex');
  try {
    await mkdir(new URL('api/', root));
    await writeFile(new URL('api/coverage.json', root), JSON.stringify([{id:'Decimal::new', doc_id:'decimal-new',status:'implemented'}]));
    for (const locale of ['en-US','zh-Hans','en-GB']) {
      await mkdir(new URL(`docs/${locale}/api/`, root), {recursive:true});
      await writeFile(new URL(`docs/${locale}/api/decimal-new.md`, root), `---\nlocale: ${locale}\nsource_revision: ${hash}\napi_id: Decimal::new\n---\n${body}`);
    }
    assert.equal(await checkMethodTranslations(root), 1);
    for (const locale of ['en-US','zh-Hans','en-GB']) {
      await writeFile(new URL(`docs/${locale}/api/decimal-new.md`, root), `---\nlocale: ${locale}\nsource_revision: ${hash}\napi_id: "Decimal::new"\n---\n${body}`);
    }
    assert.equal(await checkMethodTranslations(root), 1);
    await assert.rejects(checkMethodTranslations(root, true), /ENOENT/);
    await writeFile(new URL('docs/en-US/api/decimal-new.md', root), `---\nlocale: en-US\nsource_revision: ${hash}\napi_id: Decimal::new\n---\n${body}Changed meaning.\n`);
    await assert.rejects(checkMethodTranslations(root), /stale method translation/);
  } finally { await rm(dir, {recursive:true, force:true}); }
});
