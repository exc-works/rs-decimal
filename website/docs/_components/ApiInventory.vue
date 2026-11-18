<script setup lang="ts">
import { computed, ref } from 'vue';
import { useData, withBase } from 'vitepress';
import { referenceLabels } from '../../reference-labels.mjs';
const props = defineProps<{ labels: Record<string,string> }>();
const { lang, page } = useData();
const data = computed(() => page.value.reference.directory);
const types = computed(() => data.value.filter(item => item.group === "types"));
const captions = computed(() => referenceLabels[lang.value]);
const localesSummary = { 'en-US':'Summary', 'en-GB':'Summary', 'zh-Hans':'用途', 'zh-Hant':'用途', fr:'Description', de:'Beschreibung', ja:'概要', ko:'설명' };
const query = ref('');
const filtered = computed(() => data.value.filter((item: any) => item.id.toLowerCase().includes(query.value.toLowerCase())));
</script>
<template>
  <h2>{{ captions.types }}</h2>
  <div class="ref-types"><a v-for="item in types" :key="item.id" :href="withBase(item.href)">{{ item.id }}</a></div>
  <h2>{{ captions.all }}</h2>
  <label :for="'api-query'">{{ labels.filter }}</label>
  <input id="api-query" v-model="query" class="api-search" type="search" :placeholder="labels.filter" />
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
