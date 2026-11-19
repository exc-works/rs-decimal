<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref, watch } from 'vue';
import { useRouter, withBase } from 'vitepress';
import ExampleCode from './ExampleCode.vue';
import { RunnerClient } from '../../src/runner/client.js';
import { locales } from '../../locales.mjs';
import { formSource } from '../../src/editor/method-form.mjs';
import { encodeTransfer, HANDOFF_KEY } from '../../src/editor/transfer.mjs';
const props = defineProps<{ example: any; locale: string }>();
const router = useRouter();
const labels = computed(() => locales[props.locale]);
const values = ref<string[]>([]), storageError = ref(false), mounted = ref(false);
const storageKey = () => `rs-decimal:method:${__PLAYGROUND_CONFIG__.commit}:${props.example.id}`;
function restore() {
  values.value = props.example.inputs.map((field: any) => field.value);
  storageError.value = false;
  try {
    const text = sessionStorage.getItem(storageKey());
    if (text !== null && text.length <= 65536 * 6 + 1024) {
      const saved = JSON.parse(text);
      formSource(props.example.javascript, props.example.inputs, saved);
      values.value = saved;
    }
  } catch {}
}
const source = computed(() => {
  try { return formSource(props.example.javascript, props.example.inputs, values.value); }
  catch { return null; }
});
const runnerHost = ref<HTMLElement>();
const running = ref(false);
const output = ref<string[]>([]);
const error = ref('');
let active: RunnerClient | undefined;

function clearRun() {
  active?.dispose();
  active = undefined;
  running.value = false;
  output.value = [];
  error.value = '';
}

function displayOutput(text: string) {
  const snapshot = JSON.parse(text);
  const value = snapshot?.string_with_trailing_zeros ?? snapshot;
  return typeof value === 'string' ? value : JSON.stringify(value, null, 2);
}

async function run() {
  const script = source.value;
  if (script === null || running.value || !runnerHost.value) return;
  clearRun();
  running.value = true;
  let client: RunnerClient | undefined;
  try {
    client = new RunnerClient({ container: runnerHost.value, ...__PLAYGROUND_CONFIG__ });
    active = client;
    const result = await client.run(script, {
      onOutput: (text: string) => {
        if (active === client) output.value.push(displayOutput(text));
      },
    });
    if (active === client && result.type !== 'done') {
      error.value = [result.category, ...(result.kinds ?? []), result.message].filter(Boolean).join(' · ');
    }
  } catch (cause) {
    if (active === client) error.value = cause instanceof Error ? cause.message : String(cause);
  } finally {
    if (active === client) {
      client?.dispose();
      active = undefined;
      running.value = false;
    }
  }
}

watch(source, clearRun, { flush: 'sync' });
onBeforeUnmount(clearRun);

function save() {
  if (source.value === null) return;
  try { sessionStorage.setItem(storageKey(), JSON.stringify(values.value)); }
  catch { storageError.value = true; }
}
async function open() {
  if (source.value === null) return;
  try {
    sessionStorage.setItem(HANDOFF_KEY, encodeTransfer(source.value, __PLAYGROUND_CONFIG__.commit));
    await router.go(withBase(`/${props.locale}/playground`));
  } catch { storageError.value = true; }
}
onMounted(() => { restore(); mounted.value = true; });
watch(() => props.example.id, () => { if (mounted.value) restore(); });
</script>
<template>
  <section v-if="mounted" class="method-form" :aria-label="labels.methodTry">
    <h3>{{ labels.methodTry }}</h3>
    <p>{{ labels.methodHelp }}</p>
    <div v-for="(field, index) in example.inputs" :key="index" class="method-field">
      <label>
        <code>{{ field.label }}</code> · {{ field.type }}
        <select v-if="field.type === 'boolean'" v-model="values[index]" :data-parameter="index" @change="save">
          <option>true</option><option>false</option>
        </select>
        <textarea v-else v-model="values[index]" :data-parameter="index" rows="1" spellcheck="false" @input="save"></textarea>
      </label>
    </div>
    <p v-if="source === null" role="alert">{{ labels.methodInvalid }}</p>
    <details v-else>
      <summary>JavaScript</summary>
      <ExampleCode :source="source" language="javascript" />
    </details>
    <div class="method-actions">
      <button data-action="run-example" :disabled="source === null || running" @click="run">
        {{ running ? labels.pgRunning : labels.pgRun }}
      </button>
      <button data-action="open-example" :disabled="source === null" @click="open">{{ labels.methodOpen }}</button>
    </div>
    <div aria-live="polite">
      <template v-if="output.length">
        <h4>{{ labels.pgOutput }}</h4>
        <pre v-for="(line, i) in output" :key="i" data-example-output>{{ line }}</pre>
      </template>
      <p v-if="error" role="alert">{{ labels.pgError }} · {{ error }}</p>
    </div>
    <div ref="runnerHost"></div>
    <p v-if="storageError" role="alert">{{ labels.pgStorage }}</p>
  </section>
</template>
<style scoped>
.method-form{border:1px solid var(--vp-c-divider);border-radius:8px;padding:16px;margin:20px 0}
.method-field{margin:12px 0}.method-field code{overflow-wrap:anywhere}
textarea,select{display:block;width:100%;border:1px solid var(--vp-c-divider);border-radius:4px;padding:8px;background:var(--vp-c-bg);font:inherit}
textarea{resize:vertical}button{border-radius:6px;padding:8px 14px;background:var(--vp-c-brand-1);color:var(--vp-c-bg);font-weight:600}button:disabled{opacity:.5}
.method-actions{display:flex;flex-wrap:wrap;gap:.5rem;margin-top:.75rem}
[data-action="open-example"]{background:var(--vp-c-bg-soft);color:var(--vp-c-text-1)}
summary{cursor:pointer}
pre{white-space:pre-wrap;overflow-wrap:anywhere;font-size:13px}button:focus-visible,textarea:focus-visible,select:focus-visible{outline:2px solid var(--vp-c-brand-1);outline-offset:2px}
</style>
