// Unique aliases can be resolved even when an example calls through a local variable.
import { symbols } from '../scripts/reference-index.mjs';
export default {
  load() {
    const qualified: Record<string, string> = {};
    const methods: Record<string, string | null> = {};
    for (const item of symbols) {
      if (!['method', 'function', 'struct', 'enum', 'variant'].includes(item.kind)) continue;
      qualified[item.id] = item.slug;
      if (item.kind === 'method') {
        const name = item.id.split('::').at(-1);
        methods[name] = name in methods ? null : item.slug;
      }
    }
    return {qualified, methods};
  },
};
