import { apiPath, summary, symbolById, symbols } from './reference-index.mjs';
import { referenceLabels } from '../reference-labels.mjs';

import { taskLinks } from '../reference-tasks.mjs';

export function referencePage(page) {
  const [locale, section] = page.relativePath.split('/');
  if (!referenceLabels[locale]) return;
  const tasks = taskLinks(locale);
  if (section === 'index.md' || section === 'guide.md') {
    page.tasks = tasks.map(task => ({ ...task, href: apiPath(locale, symbolById.get(task.id)) }));
    return;
  }
  if (section !== 'api') return;
  const link = item => ({ id: item.id, href: apiPath(locale, item), group: item.group });
  const describe = item => ({ ...link(item), summary: summary(locale, item) });
  const current = symbolById.get(page.frontmatter.api_id);
  if (!current) {
    if (page.relativePath.endsWith('/api/index.md')) {
      page.reference = { directory: symbols.map(item => ({ ...describe(item), task: tasks.find(task => task.id === item.id)?.title })) };
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
