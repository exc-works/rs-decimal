// Link identifiers in prose and inline code using the same inventory as the sidebar.
import { symbols } from './reference-index.mjs';

const names = new Map();
for (const item of symbols) {
  if (!['method', 'function', 'constant', 'struct', 'enum', 'variant', 'field'].includes(item.kind)) continue;
  for (const alias of new Set([item.id, item.id.replaceAll('::', '.'), item.id.split(/::|\./).at(-1)])) {
    const entries = names.get(alias) ?? [];
    entries.push(item);
    names.set(alias, entries);
  }
}

export function referenceLinks(md) {
  md.core.ruler.after('inline', 'api-references', state => {
    const path = state.env.relativePath ?? '';
    const locale = path.split('/')[0];
    if (!/^[a-z]{2}(?:-[A-Za-z]+)?\//.test(path)) return;
    const current = symbols.find(item => path === locale + '/api/' + item.slug + '.md');
    for (const [index, block] of state.tokens.entries()) {
      if (block.type !== 'inline' || state.tokens[index - 1]?.type === 'heading_open') continue;
      let linked = false;
      block.children = block.children.flatMap(token => {
        if (token.type === 'link_open') linked = true;
        if (token.type === 'link_close') linked = false;
        if (linked || !['text', 'code_inline'].includes(token.type)) return [token];
        const parts = [];
        let end = 0;
        function fragment(content) {
          const piece = new state.Token(token.type, token.tag, 0);
          piece.content = content;
          piece.markup = token.markup;
          return piece;
        }
        for (const match of token.content.matchAll(/[A-Za-z_][\w]*(?:(?:::|\.)[A-Za-z_]\w*)*/g)) {
          const name = match[0];
          const call = token.content.slice(match.index + name.length).startsWith('(');
          if (token.type === 'text' && !call && !/[A-Z_:.]/.test(name)) continue;
          const entries = names.get(name) ?? [];
          const item = entries.find(e => e.id === name && (name.includes('::') || !e.parent))
            ?? entries.find(e => e.parent && e.parent === current?.parent)
            ?? (entries.length === 1 ? entries[0] : undefined);
          if (!item || item === current) continue;
          if (match.index > end) parts.push(fragment(token.content.slice(end, match.index)));
          const link = new state.Token('link_open', 'a', 1);
          link.attrSet('href', '/' + locale + '/api/' + item.slug + '.html');
          parts.push(link, fragment(name), new state.Token('link_close', 'a', -1));
          end = match.index + name.length;
        }
        if (!parts.length) return [token];
        if (end < token.content.length) parts.push(fragment(token.content.slice(end)));
        return parts;
      });
    }
  });
}
