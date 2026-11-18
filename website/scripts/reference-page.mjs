import { apiPath, summary, symbolById, symbols } from './reference-index.mjs';
import { referenceLabels } from '../reference-labels.mjs';

export function referencePage(page) {
  const [locale, section] = page.relativePath.split('/');
  if (section !== 'api' || !referenceLabels[locale]) return;
  const link = item => ({ id: item.id, href: apiPath(locale, item), group: item.group });
  const describe = item => ({ ...link(item), summary: summary(locale, item) });
  const current = symbolById.get(page.frontmatter.api_id);
  if (!current) {
    if (page.relativePath.endsWith('/api/index.md')) {
      page.reference = { directory: symbols.map(describe) };
    }
    return;
  }
  const parent = symbolById.get(current.parent);
  const siblings = symbols.filter(item => item.parent === current.parent && item.group === current.group);
  page.reference = {
    current: link(current),
    parent: parent && link(parent),
    siblings: siblings.map(link),
    members: symbols.filter(item => item.parent === current.id).map(describe),
  };
}
