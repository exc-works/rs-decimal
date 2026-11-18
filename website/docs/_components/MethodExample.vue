<script setup lang="ts">
import { computed, ref, watch } from 'vue';
import { referenceLabels } from '../../reference-labels.mjs';
import { data } from '../examples.data';
import MethodForm from './MethodForm.vue';
const props=defineProps<{id:string,locale:string}>();
const example=computed(()=>data[props.id]);
const language = ref('Rust');
watch(() => props.id, () => { language.value = 'Rust'; });
const result = computed(() => {
  const value = example.value?.expected;
  if (value == null || typeof value !== 'object') return JSON.stringify(value);
  return value.string_with_trailing_zeros ?? value.string;
});
const labels:Record<string,string>={'en-US':'Expected state','en-GB':'Expected state','zh-Hans':'预期状态','zh-Hant':'預期狀態',fr:'État attendu',de:'Erwarteter Zustand',ja:'期待される状態',ko:'예상 상태'};
const nativeLabels:Record<string,string>={
  'en-US':'This Rust example runs in the native test harness.', 'en-GB':'This Rust example runs in the native test harness.',
  'zh-Hans':'此 Rust 示例由原生测试执行。', 'zh-Hant':'此 Rust 範例由原生測試執行。',
  fr:'Cet exemple Rust est exécuté par les tests natifs.', de:'Dieses Rust-Beispiel wird durch native Tests ausgeführt.',
  ja:'この Rust の例はネイティブテストで実行されます。', ko:'이 Rust 예제는 네이티브 테스트에서 실행됩니다.',
};
const liveLabels:Record<string,string>={
  'en-US':'This example runs against PostgreSQL in the dedicated native CI job.', 'en-GB':'This example runs against PostgreSQL in the dedicated native CI job.',
  'zh-Hans':'此示例在专门的原生 CI 任务中连接 PostgreSQL 执行。', 'zh-Hant':'此範例在專用原生 CI 工作中連線至 PostgreSQL 執行。',
  fr:'Cet exemple est exécuté avec PostgreSQL dans une tâche CI native dédiée.', de:'Dieses Beispiel läuft mit PostgreSQL in einem eigenen nativen CI-Job.',
  ja:'この例は専用のネイティブ CI ジョブで PostgreSQL に接続して実行されます。', ko:'이 예제는 전용 네이티브 CI 작업에서 PostgreSQL에 연결하여 실행됩니다.',
};
</script>
<template>
  <section v-if="example" class="method-example">
    <h3>{{ referenceLabels[locale].example }}</h3>
    <div v-if="example.javascript.trim()" class="languages" role="group" :aria-label="referenceLabels[locale].example">
      <button v-for="name in ['Rust', 'JavaScript']" :key="name" type="button"
              :aria-pressed="language === name" @click="language = name">{{ name }}</button>
    </div>
    <pre data-example-code><code>{{ language === 'Rust' ? example.rust : example.javascript }}</code></pre>
    <p v-if="!example.javascript.trim()">{{ example.native_profile === "postgres-live" ? liveLabels[locale] : nativeLabels[locale] }}</p>
    <p v-if="result !== undefined">{{ referenceLabels[locale].result }} <code>{{ result }}</code></p>
    <details><summary>{{ labels[locale] }}</summary><pre><code>{{ JSON.stringify(example.expected,null,2) }}</code></pre></details>
    <MethodForm v-if="example.javascript.trim()" :key="example.id" :example="example" :locale="locale" />
  </section>
</template>
<style scoped>
pre{white-space:pre-wrap;overflow-wrap:anywhere;padding:14px;background:var(--vp-c-bg-soft);border-radius:6px}code{font-size:13px}
.languages { display: flex; gap: .4rem; }
.languages button, summary { cursor: pointer; }
.languages button { padding: .3rem .8rem; border: 1px solid var(--vp-c-divider); border-radius: 4px; }
.languages button[aria-pressed=true] { color: var(--vp-c-brand-1); background: var(--vp-c-bg-soft); }
</style>
