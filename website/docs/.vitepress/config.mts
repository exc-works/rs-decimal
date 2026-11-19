import { defineConfig } from 'vitepress';
import { fileURLToPath } from 'node:url';
import { execFileSync } from 'node:child_process';
import { locales } from '../../locales.mjs';

import { referenceLinks } from '../../scripts/reference-links.mjs';
import { referencePage } from '../../scripts/reference-page.mjs';

import { apiPath, operationGroups, symbols } from '../../scripts/reference-index.mjs';
import { referenceLabels } from '../../reference-labels.mjs';

function documentationSidebar(lang: string) {
  const text = locales[lang];
  const labels = referenceLabels[lang];
  return [
    { text: text.docs, items: [
      { text: text.home, link: `/${lang}/` },
      { text: text.guide, link: `/${lang}/guide` },
      { text: text.glossary, link: `/${lang}/glossary` },
    ] },
    { text: text.api, collapsed: false, items: [
      ...operationGroups.map(({ key, items }) => ({
        text: labels[key], collapsed: true,
        items: items.map(item => ({ text: item.id.slice('Decimal::'.length), link: apiPath(lang, item) })),
      })),
      { text: labels.types, collapsed: true,
        items: symbols.filter(item => item.group === 'types').map(item => ({
          text: item.id, link: apiPath(lang, item),
        })),
      },
      { text: labels.all, link: `/${lang}/api/` },
    ] },
  ];
}

export default defineConfig({
  transformPageData: referencePage,
  markdown: { config: md => md.use(referenceLinks) },
  title: 'rs-decimal',
  description: 'Exact decimal arithmetic, explicit scale and rounding.',
  base: process.env.DOCS_BASE || '/',
  cleanUrls: false,
  lastUpdated: true,
  metaChunk: true,
  vite: { publicDir: fileURLToPath(new URL('../../public', import.meta.url)), define: { __PLAYGROUND_CONFIG__: JSON.stringify({runnerUrl:(process.env.DOCS_BASE || '/') + 'runner/',commit:execFileSync('git',['rev-parse','HEAD'],{encoding:'utf8'}).trim()}) } },
  locales: Object.fromEntries(Object.entries(locales).map(([lang, text]) => [lang, {
    lang,
    label: text.label,
    title: 'rs-decimal',
    description: text.description,
    themeConfig: {
      sidebar: documentationSidebar(lang),
      outline: { label: text.onThisPage },
      docFooter: { prev: text.previous, next: text.next },
      lastUpdated: { text: text.updated },
      returnToTopLabel: text.top,
      sidebarMenuLabel: text.menu,
      darkModeSwitchLabel: text.theme,
      darkModeSwitchTitle: text.darkTheme,
      lightModeSwitchTitle: text.lightTheme,
      langMenuLabel: text.language,
      skipToContentLabel: text.skip,
    },
  }])),
  themeConfig: {
    sidebar: documentationSidebar('en-US'),
    search: { provider: 'local', options: { locales: Object.fromEntries(Object.entries(locales).map(([lang, text]) => [lang, { translations: { button: { buttonText: text.search, buttonAriaLabel: text.search }, modal: { displayDetails: text.details, resetButtonTitle: text.reset, backButtonTitle: text.back, noResultsText: text.noResults, footer: { selectText: text.select, selectKeyAriaLabel: text.enter, navigateText: text.navigate, navigateUpKeyAriaLabel: text.up, navigateDownKeyAriaLabel: text.down, closeText: text.close, closeKeyAriaLabel: text.escape } } } }])) } },
    socialLinks: [{ icon: 'github', link: 'https://github.com/exc-works/rs-decimal' }],
  },
});
