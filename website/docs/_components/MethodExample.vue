<script setup lang="ts">
import { computed } from 'vue';
import { data } from '../examples.data';
const props=defineProps<{id:string,locale:string}>();
const example=computed(()=>data[props.id]);
const labels:Record<string,string>={'en-US':'Expected state','en-GB':'Expected state','zh-Hans':'预期状态','zh-Hant':'預期狀態',fr:'État attendu',de:'Erwarteter Zustand',ja:'期待される状態',ko:'예상 상태'};
</script>
<template>
  <section v-if="example" class="method-example">
    <h3>Rust</h3><pre><code>{{ example.rust }}</code></pre>
    <h3>JavaScript</h3><pre><code>{{ example.javascript }}</code></pre>
    <h3>{{ labels[locale] }}</h3><pre><code>{{ JSON.stringify(example.expected,null,2) }}</code></pre>
  </section>
</template>
<style scoped>
pre{white-space:pre-wrap;overflow-wrap:anywhere;padding:14px;background:var(--vp-c-bg-soft);border-radius:6px}code{font-size:13px}
</style>
