---
locale: zh-Hans
source_revision: eebc95321692656eeaf68196d3bb7fece103772431d4688264c85fec9c739db3
---
# Playground

点「运行」，看看两本单价 4.50 元的笔记本加上 1.25 元运费，一共要付多少钱。输出中的应付金额是 10.25 元，付 20.00 元应找 9.75 元。把数量从 `2n` 改成 `3n`，再运行一次，应付金额会变成 14.75 元。

如果打开的是自己的草稿，可以先导出保存，再点「重置示例」载入小票。

<script setup>
import Playground from "../_components/Playground.vue";
import { locales } from "../../locales.mjs";
</script>

<Playground locale="zh-Hans" :labels="locales['zh-Hans']" />

::: details 方法示例与分享
方法页面提供共享示例的字面量参数表单。参数按 JavaScript 调用顺序排列，保留 Rust 的方法名与参数顺序。生成的代码可在此编辑器中继续修改复杂值或添加调用。方法页面的预期状态对应原始示例。本标签页切换语言后会保留表单修改。

分享功能将最多 4 KiB 的 UTF-8 代码和库构建提交号放入 URL 片段，不会将代码上传至服务器。分享链接要求相同构建版本，载入时清空之前的结果，但不会自动运行。无效链接会保留当前草稿。检查代码后点击运行。较大的脚本请导出为文件；编辑器最多接受 64 KiB。链接会包含脚本中的文本，分享前请检查代码。
:::

[Playground 资源限制](./resource-policy)
