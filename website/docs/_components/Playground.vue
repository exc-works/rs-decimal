<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref, watch } from "vue";
import { useData } from "vitepress";
import { RunnerClient } from "../../src/runner/client.js";
import { sourceValid } from "../../src/runner/protocol.js";
import { session } from "../../src/editor/state";
import {
  DEFAULT_SOURCE,
  DRAFT_KEY,
  RESULT_KEY,
  decodeDraft,
  encodeDraft,
  decodeResult,
  encodeResult,
} from "../../src/editor/draft.mjs";
const props = defineProps<{ locale: string; labels: Record<string, string> }>();
const { isDark } = useData();
const editorHost = ref<HTMLElement>();
const runnerHost = ref<HTMLElement>();
const fileInput = ref<HTMLInputElement>();
const ready = ref(false),
  notice = ref(""),
  editorError = ref(false);
const configured = Boolean(__PLAYGROUND_CONFIG__.runnerUrl);
const valid = computed(() => sourceValid(session.source));
let editor: any,
  client: any,
  changeSubscription: any,
  saveTimer: any,
  disposed = false,
  generation = 0;
function save() {
  try {
    if (valid.value)
      localStorage.setItem(DRAFT_KEY, encodeDraft(session.source));
  } catch {
    notice.value = "pgStorage";
  }
  try {
    sessionStorage.setItem(RESULT_KEY, encodeResult(session));
  } catch {}
}
function changed() {
  session.source = editor.getValue();
  clearTimeout(saveTimer);
  saveTimer = setTimeout(save, 200);
}
function stop() {
  client?.stop();
}
async function run() {
  if (!ready.value || !configured || !valid.value || session.running) return;
  const current = ++generation;
  if (["pgRestored", "pgImported"].includes(notice.value)) notice.value = "";
  session.running = true;
  session.output = [];
  session.message = "";
  session.kinds = [];
  session.status = "running";
  const start = performance.now();
  try {
    client ??= new RunnerClient({
      container: runnerHost.value,
      runnerUrl: __PLAYGROUND_CONFIG__.runnerUrl,
      commit: __PLAYGROUND_CONFIG__.commit,
    });
    const result = await client.run(session.source, {
      onOutput: (text: string) => {
        if (!disposed && current === generation) session.output.push(text);
      },
    });
    if (disposed || current !== generation) return;
    session.status = result.type === "done" ? "done" : result.category;
    session.message = result.message || "";
    session.kinds = result.kinds || [];
  } catch (error) {
    if (disposed || current !== generation) return;
    session.status = "error";
    session.message = String(error instanceof Error ? error.message : error);
    client?.dispose();
    client = undefined;
  } finally {
    if (!disposed && current === generation) {
      session.running = false;
      session.elapsed = performance.now() - start;
      save();
    }
  }
}
function reset() {
  ++generation;
  stop();
  session.running = false;
  session.output = [];
  session.message = "";
  session.kinds = [];
  session.status = "idle";
  session.elapsed = 0;
  session.source = DEFAULT_SOURCE;
  editor?.setValue(DEFAULT_SOURCE);
  notice.value = "";
  save();
}
async function importFile(event: Event) {
  const input = event.target as HTMLInputElement;
  const file = input.files?.[0];
  input.value = "";
  if (!file) return;
  if (file.size > 65536) {
    notice.value = "pgTooLarge";
    return;
  }
  try {
    const source = await file.text();
    if (!sourceValid(source)) {
      notice.value = "pgTooLarge";
      return;
    }
    if (disposed) return;
    ++generation;
    stop();
    session.running = false;
    session.status = "idle";
    session.output = [];
    session.message = "";
    session.kinds = [];
    session.elapsed = 0;
    session.source = source;
    editor?.setValue(source);
    notice.value = "pgImported";
    save();
  } catch {
    notice.value = "pgImportError";
  }
}
function exportFile() {
  const url = URL.createObjectURL(
    new Blob([session.source], { type: "text/javascript;charset=utf-8" }),
  );
  const link = document.createElement("a");
  link.href = url;
  link.download = "decimal-playground.js";
  link.click();
  setTimeout(() => URL.revokeObjectURL(url), 1000);
}
const status = computed(() =>
  session.running
    ? props.labels.pgRunning
    : session.status === "done"
      ? props.labels.pgDone
      : session.status === "idle"
        ? props.labels.pgReady
        : session.status === "cancelled"
          ? props.labels.pgStopped
          : props.labels.pgError,
);
function pretty(text: string) {
  try {
    return JSON.stringify(JSON.parse(text), null, 2);
  } catch {
    return text;
  }
}
onMounted(async () => {
  if (!session.initialized) {
    session.initialized = true;
    try {
      const text = localStorage.getItem(DRAFT_KEY);
      if (text !== null) {
        session.source = decodeDraft(text);
        notice.value = "pgRestored";
      }
    } catch {
      notice.value = "pgDraftError";
    }
    try {
      const text = sessionStorage.getItem(RESULT_KEY);
      if (text !== null) Object.assign(session, decodeResult(text));
    } catch {}
  }
  try {
    const { loadEditor } = await import("../../src/editor/loader");
    const module = await loadEditor(props.locale);
    if (disposed || !module) return;
    editor = module.createEditor(
      editorHost.value!,
      session.source,
      isDark.value,
      props.labels.pgSource,
      run,
    );
    changeSubscription = editor.onChange(changed);
    ready.value = true;
  } catch {
    editorError.value = true;
  }
});
watch(isDark, (value) => editor?.setTheme(value));
onBeforeUnmount(() => {
  disposed = true;
  ++generation;
  clearTimeout(saveTimer);
  if (session.running) {
    session.running = false;
    session.status = "cancelled";
  }
  save();
  client?.dispose();
  changeSubscription?.dispose();
  editor?.dispose();
});
</script>
<template>
  <section
    class="playground"
    :data-editor-ready="ready"
    :aria-label="labels.playground"
  >
    <p v-if="!configured" class="pg-notice" role="status">
      {{ labels.pgUnavailable }}
    </p>
    <div class="pg-toolbar" role="group" :aria-label="labels.pgControls">
      <button
        data-action="run"
        class="pg-primary"
        :disabled="!ready || !configured || !valid || session.running"
        @click="run"
      >
        {{ labels.pgRun }}
      </button>
      <button data-action="stop" :disabled="!session.running" @click="stop">
        {{ labels.pgStop }}
      </button>
      <button data-action="reset" :disabled="!ready" @click="reset">
        {{ labels.pgReset }}
      </button>
      <button
        data-action="import"
        :disabled="!ready"
        @click="fileInput?.click()"
      >
        {{ labels.pgImport }}
      </button>
      <button data-action="export" :disabled="!ready" @click="exportFile">
        {{ labels.pgExport }}
      </button>
      <input
        ref="fileInput"
        class="pg-file"
        type="file"
        accept=".js,.mjs,.txt"
        :aria-label="labels.pgImport"
        @change="importFile"
      />
    </div>
    <p class="pg-status" aria-live="polite">
      {{ status }}
      <span v-if="session.elapsed">· {{ session.elapsed.toFixed(0) }} ms</span>
    </p>
    <p v-if="notice" role="status">{{ labels[notice] }}</p>
    <p v-if="!valid" role="alert">{{ labels.pgTooLarge }}</p>
    <p v-if="!ready && !editorError">{{ labels.pgLoading }}</p>
    <p v-if="editorError" role="alert">{{ labels.pgEditorError }}</p>
    <div ref="editorHost" class="pg-editor" :aria-label="labels.pgSource"></div>
    <p class="pg-help">{{ labels.pgHelp }}</p>
    <h2>{{ labels.pgOutput }}</h2>
    <p v-if="!session.output.length">{{ labels.pgEmpty }}</p>
    <div class="pg-output" aria-live="polite" :aria-label="labels.pgOutput">
      <pre v-for="(text, index) in session.output" :key="index">{{
        pretty(text)
      }}</pre>
    </div>
    <div v-if="session.message" role="alert" class="pg-error">
      <strong>{{ labels.pgError }} · {{ session.status }}</strong>
      <p v-if="session.kinds.length">{{ session.kinds.join(", ") }}</p>
      <pre>{{ session.message }}</pre>
    </div>
    <div ref="runnerHost"></div>
  </section>
</template>
<style scoped>
.pg-intro {
  color: var(--vp-c-text-2);
}
.pg-toolbar {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  margin: 20px 0 8px;
}
.pg-toolbar button {
  border: 1px solid var(--vp-c-divider);
  padding: 7px 13px;
  border-radius: 6px;
  font-weight: 600;
  background: var(--vp-c-bg-soft);
}
.pg-toolbar button:focus-visible {
  outline: 2px solid var(--vp-c-brand-1);
  outline-offset: 2px;
}
.pg-toolbar button.pg-primary {
  background: var(--vp-c-brand-1);
  color: var(--vp-c-bg);
}
.pg-toolbar button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
.pg-editor {
  height: 420px;
  border: 1px solid var(--vp-c-divider);
  border-radius: 8px;
  overflow: hidden;
}
.pg-help,
.pg-status {
  font-size: 13px;
  color: var(--vp-c-text-2);
}
.pg-notice {
  padding: 12px;
  border-left: 3px solid var(--vp-c-brand-1);
  background: var(--vp-c-bg-soft);
}
.pg-file {
  display: none;
}
.pg-output pre,
.pg-error pre {
  white-space: pre-wrap;
  overflow-wrap: anywhere;
  font-size: 13px;
  padding: 14px;
  background: var(--vp-c-bg-soft);
  border-radius: 6px;
}
.pg-error {
  border-left: 3px solid var(--vp-c-danger-1);
  padding-left: 14px;
  margin-top: 16px;
}
@media (max-width: 640px) {
  .pg-editor {
    height: 360px;
  }
  .pg-toolbar button {
    flex: 1 0 auto;
  }
}
</style>
