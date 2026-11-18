<script setup lang="ts">
import { computed } from 'vue';
import { data } from '../examples.data';
const props=defineProps<{id:string,locale:string}>();
const example=computed(()=>data[props.id]);
const labels:Record<string,string>={'en-US':'Expected state','en-GB':'Expected state','zh-Hans':'预期状态','zh-Hant':'預期狀態',fr:'État attendu',de:'Erwarteter Zustand',ja:'期待される状態',ko:'예상 상태'};
const nativeLabels:Record<string,string>={
  'en-US':'This Rust example runs in the native test harness.', 'en-GB':'This Rust example runs in the native test harness.',
  'zh-Hans':'此 Rust 示例由原生测试执行。', 'zh-Hant':'此 Rust 範例由原生測試執行。',
  fr:'Cet exemple Rust est exécuté par les tests natifs.', de:'Dieses Rust-Beispiel wird durch native Tests ausgeführt.',
  ja:'この Rust の例はネイティブテストで実行されます。', ko:'이 Rust 예제는 네이티브 테스트에서 실행됩니다.',
};
</script>
<template>
  <section v-if="example" class="method-example">
    <h3>Rust</h3><pre><code>{{ example.rust }}</code></pre>
    <template v-if="example.javascript.trim()"><h3>JavaScript</h3><pre><code>{{ example.javascript }}</code></pre></template>
    <p v-else>{{ nativeLabels[locale] }}</p>
    <h3>{{ labels[locale] }}</h3><pre><code>{{ JSON.stringify(example.expected,null,2) }}</code></pre>
  </section>
</template>
<style scoped>
pre{white-space:pre-wrap;overflow-wrap:anywhere;padding:14px;background:var(--vp-c-bg-soft);border-radius:6px}code{font-size:13px}
</style>
