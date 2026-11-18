---
locale: zh-Hans
source_revision: a8248cf86b647c5ae7b7b6b6d557a514ea0e7a1ff72bdd8a4e20a36c0572eb5a
---
# Playground

用 JavaScript 调用与 Rust 同名的方法。点击运行，检查精确数值和状态。

<script setup>
import Playground from "../_components/Playground.vue";
import { locales } from "../../locales.mjs";
</script>

<Playground locale="zh-Hans" :labels="locales['zh-Hans']" />
