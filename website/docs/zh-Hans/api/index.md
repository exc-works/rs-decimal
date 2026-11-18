---
locale: zh-Hans
source_revision: b0ed31c9fd5b925f380c834184c84a74d9c454dd7b30cb7cce9e6b371d0d2219
---
# API 目录

本清单从 Rust 源码提取，包含方法、类型、字段、枚举项、trait 和 re-export。条目总数不等于可运行方法数量。计划中条目尚未实现；仅 Rust 条目提供原生用法示例，不在浏览器执行。

<script setup>
import ApiInventory from "../../_components/ApiInventory.vue";
import { locales } from "../../../locales.mjs";
</script>

<ApiInventory :labels="locales['zh-Hans']" />
