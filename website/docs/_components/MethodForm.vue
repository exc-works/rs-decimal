<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue';
import { useRouter, withBase } from 'vitepress';
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
    <pre v-else><code>{{ source }}</code></pre>
    <button data-action="open-example" :disabled="source === null" @click="open">{{ labels.methodOpen }}</button>
    <p v-if="storageError" role="alert">{{ labels.pgStorage }}</p>
  </section>
</template>
<style scoped>
.method-form{border:1px solid var(--vp-c-divider);border-radius:8px;padding:16px;margin:20px 0}
.method-field{margin:12px 0}.method-field code{overflow-wrap:anywhere}
textarea,select{display:block;width:100%;border:1px solid var(--vp-c-divider);border-radius:4px;padding:8px;background:var(--vp-c-bg);font:inherit}
textarea{resize:vertical}button{border-radius:6px;padding:8px 14px;background:var(--vp-c-brand-1);color:var(--vp-c-bg);font-weight:600}button:disabled{opacity:.5}
pre{white-space:pre-wrap;overflow-wrap:anywhere;font-size:13px}button:focus-visible,textarea:focus-visible,select:focus-visible{outline:2px solid var(--vp-c-brand-1);outline-offset:2px}
</style>
