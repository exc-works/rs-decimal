<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref } from 'vue';
import { useRouter, withBase } from 'vitepress';
import { RoundingMode } from '../../src/bridge/index.js';
import { RunnerClient, runnerErrorMessage } from '../../src/runner/client.js';
import { locales } from '../../locales.mjs';
import { formSource } from '../../src/editor/method-form.mjs';
import { encodeTransfer, HANDOFF_KEY } from '../../src/editor/transfer.mjs';
const props = defineProps<{ example: any; locale: string }>();
const router = useRouter();
const labels = computed(() => locales[props.locale]);
const emit = defineEmits<{edit: [values: string[]]}>();
const fieldLabels = {mode:'fieldMode', prec:'fieldPrec', input:'fieldInput', value:'fieldValue', a:'fieldLeft', b:'fieldRight'};
const debounceMs = 200;
const draft = ref<string[]>(props.example.inputs.map(field => field.value));
const mounted = ref(false), storageError = ref(false);
const runnerHost = ref<HTMLElement>();
const busy = ref(false), invalid = ref(false);
const output = ref<string[]>([]), error = ref('');
let dirty = false;
let timer: ReturnType<typeof setTimeout> | undefined;
let client: RunnerClient | undefined;
let generation = 0;
const storageKey = () => `rs-decimal:method:${__PLAYGROUND_CONFIG__.commit}:${props.example.id}`;
const scriptFor = (values: string[]) => formSource(props.example.javascript, props.example.inputs, values);

function restore() {
  try {
    const text = sessionStorage.getItem(storageKey());
    if (text !== null && text.length <= 65536 * 6 + 1024) {
      const saved = JSON.parse(text);
      scriptFor(saved);
      draft.value = saved;
    }
  } catch {}
}

function persist() {
  if (!dirty) return;
  try { sessionStorage.setItem(storageKey(), JSON.stringify(draft.value)); dirty = false; }
  catch { storageError.value = true; }
}

function cancelWork() {
  clearTimeout(timer);
  generation++;
  client?.stop();
}

function prepare() {
  try {
    const script = scriptFor(draft.value);
    invalid.value = false;
    emit('edit', draft.value.slice());
    persist();
    return script;
  } catch {
    invalid.value = true;
    output.value = [];
    error.value = '';
    busy.value = false;
    return null;
  }
}

function displayOutput(text: string) {
  const snapshot = JSON.parse(text);
  const value = snapshot?.string_with_trailing_zeros ?? snapshot;
  return typeof value === 'string' ? value : JSON.stringify(value, null, 2);
}

async function run() {
  cancelWork();
  const script = prepare();
  if (script === null || !runnerHost.value) return;
  busy.value = true;
  const current = generation;
  const lines: string[] = [];
  try {
    client ??= new RunnerClient({ container: runnerHost.value, ...__PLAYGROUND_CONFIG__ });
    const result = await client.run(script, { onOutput: (text: string) => lines.push(displayOutput(text)) });
    if (generation !== current) return;
    output.value = lines;
    error.value = result.type === 'done' ? '' :
      [result.category, ...(result.kinds ?? []), result.message].filter(Boolean).join(' · ');
  } catch (cause) {
    if (generation === current) {
      output.value = lines;
      error.value = runnerErrorMessage(cause, labels.value);
      client?.dispose();
      client = undefined;
    }
  } finally {
    if (generation === current) busy.value = false;
  }
}

function update(index: number, event: Event, delay: number) {
  cancelWork();
  draft.value[index] = (event.target as HTMLInputElement).value;
  dirty = true;
  busy.value = true;
  if (!(event as InputEvent).isComposing) timer = setTimeout(run, delay);
}

function savePendingDraft() {
  if (!dirty) return;
  try { scriptFor(draft.value); } catch { return; }
  persist();
}

async function open() {
  const script = prepare();
  if (script === null) return;
  try {
    sessionStorage.setItem(HANDOFF_KEY, encodeTransfer(script, __PLAYGROUND_CONFIG__.commit));
    await router.go(withBase(`/${props.locale}/playground`));
  } catch { storageError.value = true; }
}
onMounted(() => {
  restore();
  mounted.value = true;
  timer = setTimeout(run, 0);
  window.addEventListener('pagehide', savePendingDraft);
});
onBeforeUnmount(() => {
  cancelWork();
  client?.dispose();
  savePendingDraft();
  window.removeEventListener('pagehide', savePendingDraft);
});
</script>
<template>
  <section v-if="mounted" class="method-form" :aria-label="labels.methodTry">
    <h3>{{ labels.methodTry }}</h3>
    <p v-if="example.inputs.length">{{ labels.methodHelp }}</p>
    <div v-for="(field, index) in example.inputs" :key="index" class="method-field">
      <label>
        {{ labels[fieldLabels[field.label]] ?? field.label }}
        <select v-if="field.type === 'rounding' || field.type === 'boolean'" :value="draft[index]" :data-parameter="index" @change="update(index, $event, 0)">
          <option v-for="choice in field.type === 'rounding' ? Object.keys(RoundingMode) : ['true','false']" :key="choice">{{ choice }}</option>
        </select>
        <textarea v-else :value="draft[index]" :data-parameter="index" rows="1" spellcheck="false" @input="update(index, $event, debounceMs)" @compositionend="update(index, $event, debounceMs)"></textarea>
      </label>
    </div>
    <h4>{{ labels.expectedResult }}</h4>
    <div class="method-result" aria-live="polite" :aria-busy="busy" :aria-label="labels.expectedResult" role="region" tabindex="0">
      <pre v-for="(line, i) in output" :key="i" data-example-output>{{ line }}</pre>
      <p v-if="invalid" role="alert">{{ labels.methodInvalid }}</p>
      <template v-else-if="error">
        <p role="alert">{{ labels.pgError }} · {{ error }}</p>
        <button data-action="run-example" :disabled="busy" @click="run">{{ labels.pgRun }}</button>
      </template>
    </div>
    <div class="method-actions">
      <button data-action="open-example" :disabled="invalid && !busy" @click="open">{{ labels.methodOpen }}</button>
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
.method-result { box-sizing: border-box; block-size: 6rem; overflow: auto; padding: .5rem .75rem; background: var(--vp-c-bg-soft); border-radius: 6px; scrollbar-gutter: stable; }
.method-result pre, .method-result p { margin: 0; }
.method-result[aria-busy="true"] > * { opacity: .7; }
.method-result:focus-visible { outline: 2px solid var(--vp-c-brand-1); }
pre{white-space:pre-wrap;overflow-wrap:anywhere;font-size:13px}button:focus-visible,textarea:focus-visible,select:focus-visible{outline:2px solid var(--vp-c-brand-1);outline-offset:2px}
</style>
