<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import { useData, withBase } from 'vitepress';
import { searchReference } from '../../scripts/reference-search.mjs';
import { referenceLabels } from '../../reference-labels.mjs';
const props = defineProps<{ labels: Record<string,string> }>();
const { lang, page } = useData();
const data = computed(() => page.value.reference?.directory ?? []);
const types = computed(() => data.value.filter(item => item.group === "types"));
const captions = computed(() => referenceLabels[lang.value]);
const localesSummary = { 'en-US':'Summary', 'en-GB':'Summary', 'zh-Hans':'用途', 'zh-Hant':'用途', fr:'Description', de:'Beschreibung', ja:'概要', ko:'설명' };
const ready = ref(false);
onMounted(() => { ready.value = true; });
const query = ref('');
const filtered = computed(() => searchReference(data.value, query.value, captions.value));
</script>
<template>
  <label :for="'api-query'">{{ labels.search }}</label>
  <input id="api-query" :disabled="!ready" v-model="query" class="api-search" type="search" :placeholder="labels.search" />
  <template v-if="!query.trim()">
    <h2>{{ captions.operations }}</h2>
    <details v-for="group in page.reference.operations" :key="group.key" class="operation-group">
      <summary>{{ captions[group.key] }}</summary>
      <dl>
        <template v-for="item in group.items" :key="item.id">
          <dt><a :href="withBase(item.href)"><code>{{ item.id.slice('Decimal::'.length) }}</code></a></dt>
          <dd>{{ item.summary }}</dd>
        </template>
      </dl>
    </details>
  <h2>{{ captions.types }}</h2>
  <div class="ref-types"><a v-for="item in types" :key="item.id" :href="withBase(item.href)">{{ item.id }}</a></div>
  </template>
  <h2>{{ captions.all }}</h2>
  <p aria-live="polite">{{ filtered.length }} / {{ data.length }}</p>
  <div class="api-table-wrap">
    <table>
      <thead><tr><th>{{ labels.method }}</th><th>{{ localesSummary[lang] }}</th></tr></thead>
      <tbody><tr v-for="item in filtered" :key="item.id">
        <td><a :href="withBase(item.href)"><code>{{ item.id }}</code></a></td>
        <td>{{ item.summary }}</td>
      </tr></tbody>
    </table>
  </div>
</template>
