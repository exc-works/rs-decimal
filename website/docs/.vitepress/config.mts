import { defineConfig } from 'vitepress';
import { locales } from '../../locales.mjs';

export default defineConfig({
  title: 'rs-decimal',
  description: 'Exact decimal arithmetic, explicit scale and rounding.',
  base: process.env.DOCS_BASE || '/',
  cleanUrls: false,
  lastUpdated: true,
  locales: Object.fromEntries(Object.entries(locales).map(([lang, text]) => [lang, {
    lang,
    label: text.label,
    title: 'rs-decimal',
    description: text.description,
    themeConfig: {
      nav: [
        { text: text.guide, link: `/${lang}/guide` },
        { text: text.api, link: `/${lang}/api/` },
        { text: text.glossary, link: `/${lang}/glossary` },
      ],
      sidebar: [{ text: text.docs, items: [
        { text: text.home, link: `/${lang}/` },
        { text: text.guide, link: `/${lang}/guide` },
        { text: text.api, link: `/${lang}/api/` },
        { text: text.glossary, link: `/${lang}/glossary` },
      ] }],
      outline: { label: text.onThisPage },
      docFooter: { prev: text.previous, next: text.next },
      lastUpdated: { text: text.updated },
      returnToTopLabel: text.top,
      sidebarMenuLabel: text.menu,
      darkModeSwitchLabel: text.theme,
      langMenuLabel: text.language,
      skipToContentLabel: text.skip,
    },
  }])),
  themeConfig: {
    search: { provider: 'local', options: { locales: Object.fromEntries(Object.entries(locales).map(([lang, text]) => [lang, { translations: { button: { buttonText: text.search, buttonAriaLabel: text.search }, modal: { displayDetails: text.details, resetButtonTitle: text.reset, backButtonTitle: text.back, noResultsText: text.noResults, footer: { selectText: text.select, selectKeyAriaLabel: text.enter, navigateText: text.navigate, navigateUpKeyAriaLabel: text.up, navigateDownKeyAriaLabel: text.down, closeText: text.close, closeKeyAriaLabel: text.escape } } } }])) } },
    socialLinks: [{ icon: 'github', link: 'https://github.com/exc-works/rs-decimal' }],
  },
});
