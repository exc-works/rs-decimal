---
locale: zh-Hant
source_revision: a8248cf86b647c5ae7b7b6b6d557a514ea0e7a1ff72bdd8a4e20a36c0572eb5a
---
# Playground

用 JavaScript 呼叫與 Rust 同名的方法。點選執行，檢查精確數值和狀態。

<script setup>
import Playground from "../_components/Playground.vue";
import { locales } from "../../locales.mjs";
</script>

<Playground locale="zh-Hant" :labels="locales['zh-Hant']" />
