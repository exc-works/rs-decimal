<script setup lang="ts">
import { computed, ref } from 'vue';
import { useData, withBase } from 'vitepress';
import { data } from '../api.data';
const props = defineProps<{ labels: Record<string,string> }>();
const { lang } = useData();
const referenceLocale = computed(() => ['en-US','zh-Hans'].includes(lang.value) ? lang.value : 'en-US');
const fallbackLabels: Record<string,string> = {
  'zh-Hant':'英文參考文件', 'en-GB':'English (US) reference', fr:'Référence en anglais',
  ko:'영어 참고 문서', ja:'英語のリファレンス', de:'Englische Referenz',
};
const query = ref('');
const filtered = computed(() => data.filter((item: any) => item.id.toLowerCase().includes(query.value.toLowerCase())));
</script>
<template>
  <label :for="'api-query'">{{ labels.filter }}</label>
  <input id="api-query" v-model="query" class="api-search" type="search" :placeholder="labels.filter" />
  <p aria-live="polite">{{ filtered.length }} / {{ data.length }}</p>
  <div class="api-table-wrap">
    <table>
      <thead><tr><th>{{ labels.method }}</th><th>{{ labels.status }}</th><th>{{ labels.mapping }}</th></tr></thead>
      <tbody><tr v-for="item in filtered" :key="item.id">
        <td>
          <a v-if="item.status === 'implemented'" :href="withBase(`/${referenceLocale}/api/${item.doc_id}.html`)">
            <code>{{ item.id }}</code><span v-if="referenceLocale !== lang"> ({{ fallbackLabels[lang] }})</span>
          </a>
          <code v-else>{{ item.id }}</code>
        </td>
        <td>{{ item.status === 'implemented' ? labels.implemented : labels.planned }}</td>
        <td>{{ labels[item.disposition] }}</td>
      </tr></tbody>
    </table>
  </div>
</template>
