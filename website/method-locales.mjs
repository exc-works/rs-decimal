// Used by both the translation gate and catalogue links.
export function methodLocales(owner, groups = {}) {
  return [...new Set(['en-US', 'zh-Hans', 'en-GB', ...Object.keys(groups).filter(locale => groups[locale].includes('*') || groups[locale].includes(owner))])];
}
