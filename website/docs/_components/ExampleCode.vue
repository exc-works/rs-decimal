<script setup lang="ts">
import { computed } from 'vue';
import { useData, withBase } from 'vitepress';
import { data as targets } from '../example-links.data';

const props = defineProps<{ source: string; language: 'rust' | 'javascript'; parts?: {text: string; changed?: boolean}[] }>();
const highlight = (source: string) => hljs.highlight(source, { language: props.language }).value;
const { lang } = useData();
function linked(source: string) {
  const tokens = /"(?:\\.|[^"\\])*"|'(?:\\.|[^'\\])*'|\/\/[^\n]*|\/\*[\s\S]*?\*\/|[A-Za-z_]\w*(?:(?:::|\.)[A-Za-z_]\w*)*/g;
  let markup = '', offset = 0;
  for (const match of source.matchAll(tokens)) {
    const name = match[0];
    if (!/^[A-Za-z_]/.test(name)) continue;
    const qualified = name.replaceAll('.', '::');
    const call = source[match.index + name.length] === '(';
    let slug = Object.hasOwn(targets.qualified, qualified) && (call || /[A-Z]/.test(qualified)) ? targets.qualified[qualified] : null;
    let start = match.index;
    if (!slug && call && (name.includes('.') || source[start - 1] === '.')) {
      const method = name.split('.').at(-1)!;
      slug = Object.hasOwn(targets.methods, method) ? targets.methods[method] : null;
      start += name.length - method.length;
    }
    if (!slug) continue;
    const end = match.index + name.length;
    markup += highlight(source.slice(offset, start));
    markup += '<a href="' + withBase('/' + lang.value + '/api/' + slug + '.html') + '">' + highlight(source.slice(start, end)) + '</a>';
    offset = end;
  }
  return markup + highlight(source.slice(offset));
}
const markup = computed(() => {
  if (!props.parts?.some(part => part.changed)) return linked(props.source);
  return props.parts.map(part => part.changed ? '<mark>' + linked(part.text) + '</mark>' : linked(part.text)).join('');
});
</script>

<script lang="ts">
import hljs from 'highlight.js/lib/core';
import javascript from 'highlight.js/lib/languages/javascript';
import rust from 'highlight.js/lib/languages/rust';
hljs.registerLanguage('javascript', javascript);
hljs.registerLanguage('rust', rust);
</script>

<template>
  <pre class="example-code"><code v-html="markup"></code></pre>
</template>

<style>
.example-code {
  padding: 14px;
  background: var(--vp-code-block-bg);
  border-radius: 6px;
  white-space: pre-wrap;
  overflow-wrap: anywhere;
}
.vp-doc .example-code code { color: var(--vp-c-text-1); font-size: 13px; }
.example-code .hljs-keyword, .example-code .hljs-literal { color: var(--example-keyword); }
.example-code .hljs-string, .example-code .hljs-regexp { color: var(--example-string); }
.example-code .hljs-number, .example-code .hljs-built_in, .example-code .hljs-type { color: var(--example-value); }
.example-code .hljs-title, .example-code .hljs-meta { color: var(--example-title); }
.example-code mark { background: var(--vp-c-warning-soft); border-bottom: 2px solid var(--vp-c-warning-1); color: inherit; }
.example-code .hljs-comment { color: var(--vp-c-text-2); }
.example-code { --example-keyword: #a62648; --example-string: #176038; --example-value: #0656a0; --example-title: #6941a5; }
.dark .example-code { --example-keyword: #ff9da9; --example-string: #98d7a4; --example-value: #8fc7ff; --example-title: #c7abff; }
</style>
