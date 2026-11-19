<script setup lang="ts">
import { computed } from 'vue';

const props = defineProps<{ source: string; language: 'rust' | 'javascript' }>();
const markup = computed(() => hljs.highlight(props.source, { language: props.language }).value);
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
.example-code .hljs-comment { color: var(--vp-c-text-2); }
.example-code { --example-keyword: #a62648; --example-string: #176038; --example-value: #0656a0; --example-title: #6941a5; }
.dark .example-code { --example-keyword: #ff9da9; --example-string: #98d7a4; --example-value: #8fc7ff; --example-title: #c7abff; }
</style>
