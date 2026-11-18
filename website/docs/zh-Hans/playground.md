---
locale: zh-Hans
source_revision: fbadb341fac6ded3986e830a05797cf96857f33d92f3224a113dd3d05ba9eb85
---
# Playground

用 JavaScript 调用与 Rust 同名的方法。点击运行，检查精确数值和状态。

<script setup>
import Playground from "../_components/Playground.vue";
import { locales } from "../../locales.mjs";
</script>

<Playground locale="zh-Hans" :labels="locales['zh-Hans']" />

方法页面提供共享示例的字面量参数表单。参数按 JavaScript 调用顺序排列，保留 Rust 的方法名与参数顺序。生成的代码可在此编辑器中继续修改复杂值或添加调用。方法页面的预期状态对应原始示例。本标签页切换语言后会保留表单修改。

分享功能将最多 4 KiB 的 UTF-8 代码和库构建提交号放入 URL 片段，不会将代码上传至服务器。分享链接要求相同构建版本，载入时清空之前的结果，但不会自动运行。无效链接会保留当前草稿。检查代码后点击运行。较大的脚本请导出为文件；编辑器最多接受 64 KiB。链接会包含脚本中的文本，分享前请检查代码。
