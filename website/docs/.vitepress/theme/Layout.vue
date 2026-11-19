<script setup>
import DefaultTheme from 'vitepress/theme';
import { computed, onMounted, watch } from 'vue';
import { useData, withBase } from 'vitepress';
import { referenceLabels } from '../../../reference-labels.mjs';
import { locales } from '../../../locales.mjs';
const { page, lang } = useData();
onMounted(() => {
  const preference = 'rs-decimal:docs-language';
  if (page.value.relativePath === 'index.md') {
    let saved;
    try { saved = localStorage.getItem(preference); } catch { /* Storage is optional. */ }
    const preferred = navigator.languages.map(tag => {
      const locale = new Intl.Locale(tag);
      if (locale.language === 'zh') return 'zh-' + locale.maximize().script;
      if (locale.language === 'en') return locale.region === 'GB' ? 'en-GB' : 'en-US';
      return locale.language;
    });
    const target = [saved, ...preferred].find(id => Object.hasOwn(locales, id)) ?? 'en-US';
    location.replace(withBase('/' + target + '/') + location.search + location.hash);
  }
  watch(() => page.value.relativePath, path => {
    const locale = path.split('/')[0];
    if (Object.hasOwn(locales, locale)) {
      try { localStorage.setItem(preference, locale); } catch { /* Storage is optional. */ }
    }
  }, { immediate: true });
});
const reference = computed(() => page.value.reference);
const labels = computed(() => referenceLabels[lang.value]);
const catalogue = computed(() => withBase('/' + lang.value + '/api/'));
const groups = computed(() => {
  const result = new Map();
  for (const member of reference.value?.members ?? []) {
    if (!result.has(member.group)) result.set(member.group, []);
    result.get(member.group).push(member);
  }
  return Array.from(result);
});
</script>

<template>
  <DefaultTheme.Layout>
    <template #sidebar-nav-before>
      <nav v-if="reference?.current && !reference.operation" class="ref-context" :aria-label="locales[lang].api">
        <a :href="catalogue">{{ locales[lang].api }}</a>
        <a v-if="reference.parent" :href="withBase(reference.parent.href)">{{ reference.parent.id }}</a>
        <strong>{{ labels[reference.current.group] }}</strong>
        <a v-for="item in reference.siblings" :key="item.id" :href="withBase(item.href)"
           :aria-current="item.id === reference.current.id ? 'page' : undefined">{{ item.id }}</a>
      </nav>
    </template>
    <template #doc-before>
      <nav v-if="reference?.current" class="ref-path" :aria-label="locales[lang].api">
        <a :href="catalogue">{{ locales[lang].api }}</a>
        <template v-if="reference.parent"> / <a :href="withBase(reference.parent.href)">{{ reference.parent.id }}</a></template>
        / {{ labels[reference.operation ?? reference.current.group] }}
      </nav>
      <nav v-if="groups.length" class="ref-jump" :aria-label="labels.members">
        <a v-for="[group] in groups" :key="group" :href="'#members-' + group">{{ labels[group] }}</a>
      </nav>
    </template>
    <template #doc-after>
      <nav v-if="page.tasks" class="ref-tasks" :aria-label="locales[lang].guide">
        <a v-for="task in page.tasks" :key="task.id" :href="withBase(task.href)">{{ task.title }}</a>
      </nav>
      <section v-for="[group, items] in groups" :key="group" class="ref-group" :id="'members-' + group">
        <h2>{{ labels[group] }}</h2>
        <dl><template v-for="item in items" :key="item.id">
          <dt><a :href="withBase(item.href)">{{ item.id }}</a></dt>
          <dd>{{ item.summary }}</dd>
        </template></dl>
      </section>
    </template>
  </DefaultTheme.Layout>
</template>
