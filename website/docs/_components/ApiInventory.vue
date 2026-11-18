<script setup lang="ts">
import { computed, ref } from 'vue';
import { data } from '../api.data';
const props = defineProps<{ labels: Record<string,string> }>();
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
        <td><code>{{ item.id }}</code></td>
        <td>{{ item.status === 'implemented' ? labels.implemented : labels.planned }}</td>
        <td>{{ labels[item.disposition] }}</td>
      </tr></tbody>
    </table>
  </div>
</template>
