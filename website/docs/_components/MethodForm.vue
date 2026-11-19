<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref, watch } from 'vue';
import { useRouter, withBase } from 'vitepress';
import { RoundingMode } from '../../src/bridge/index.js';
import { RunnerClient } from '../../src/runner/client.js';
import { locales } from '../../locales.mjs';
import { formSource } from '../../src/editor/method-form.mjs';
import { encodeTransfer, HANDOFF_KEY } from '../../src/editor/transfer.mjs';
const props = defineProps<{ example: any; locale: string; values: string[]; source: string | null }>();
const router = useRouter();
const labels = computed(() => locales[props.locale]);
const emit = defineEmits<{edit: [values: string[]]}>();
const fieldLabels = {mode:'fieldMode', prec:'fieldPrec', input:'fieldInput', value:'fieldValue', a:'fieldLeft', b:'fieldRight'};
const storageError = ref(false), mounted = ref(false);
const storageKey = () => `rs-decimal:method:${__PLAYGROUND_CONFIG__.commit}:${props.example.id}`;
function restore() {
  storageError.value = false;
  try {
    const text = sessionStorage.getItem(storageKey());
    if (text !== null && text.length <= 65536 * 6 + 1024) {
      const saved = JSON.parse(text);
      formSource(props.example.javascript, props.example.inputs, saved);
      emit('edit', saved);
    }
  } catch {}
}
const runnerHost = ref<HTMLElement>();
const running = ref(false);
const output = ref<string[]>([]);
const error = ref('');
let active: RunnerClient | undefined;
let timer: ReturnType<typeof setTimeout> | undefined;

function clearRun() {
  clearTimeout(timer);
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
  const script = props.source;
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

watch([mounted, () => props.source], () => {
  clearRun();
  if (mounted.value && props.source !== null) timer = setTimeout(run, 250);
}, { flush: 'sync' });
onBeforeUnmount(clearRun);

function update(index: number, event: Event) {
  const next = props.values.slice();
  next[index] = (event.target as HTMLInputElement).value;
  emit('edit', next);
  try { formSource(props.example.javascript, props.example.inputs, next); }
  catch { return; }
  try { sessionStorage.setItem(storageKey(), JSON.stringify(next)); }
  catch { storageError.value = true; }
}
async function open() {
  if (props.source === null) return;
  try {
    sessionStorage.setItem(HANDOFF_KEY, encodeTransfer(props.source, __PLAYGROUND_CONFIG__.commit));
    await router.go(withBase(`/${props.locale}/playground`));
  } catch { storageError.value = true; }
}
onMounted(() => { restore(); mounted.value = true; });
watch(() => props.example.id, () => { if (mounted.value) restore(); });
</script>
<template>
  <section v-if="mounted" class="method-form" :aria-label="labels.methodTry">
    <h3>{{ labels.methodTry }}</h3>
    <p v-if="example.inputs.length">{{ labels.methodHelp }}</p>
    <div v-for="(field, index) in example.inputs" :key="index" class="method-field">
      <label>
        {{ labels[fieldLabels[field.label]] ?? field.label }}
        <select v-if="field.type === 'rounding' || field.type === 'boolean'" :value="values[index]" :data-parameter="index" @change="update(index, $event)">
          <option v-for="choice in field.type === 'rounding' ? Object.keys(RoundingMode) : ['true','false']" :key="choice">{{ choice }}</option>
        </select>
        <textarea v-else :value="values[index]" :data-parameter="index" rows="1" spellcheck="false" @input="update(index, $event)"></textarea>
      </label>
    </div>
    <p v-if="source === null" role="alert">{{ labels.methodInvalid }}</p>
    <div aria-live="polite">
      <h4>{{ labels.expectedResult }}</h4>
      <pre v-for="(line, i) in output" :key="i" data-example-output>{{ line }}</pre>
      <p v-if="error" role="alert">{{ labels.pgError }} · {{ error }}</p>
      <p v-else-if="source !== null && !output.length">{{ labels.pgRunning }}</p>
    </div>
    <div class="method-actions">
      <button v-if="error" data-action="run-example" :disabled="source === null || running" @click="run">{{ labels.pgRun }}</button>
      <button data-action="open-example" :disabled="source === null" @click="open">{{ labels.methodOpen }}</button>
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
