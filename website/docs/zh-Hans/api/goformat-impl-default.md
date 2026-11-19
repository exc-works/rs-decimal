---
locale: zh-Hans
source_revision: 07a3f028e4aca5abdb58974bbb6c24c9b9018814af668c73f9dc49c66b12687d
api_id: GoFormat::impl Default
---
# GoFormat::impl Default

使用 Rust Default 实现。

```rust
impl Default for GoFormat
```

## 参数、返回值及常规用法

GoFormat 是小型 Copy 字段结构体。Clone/Copy 保留选项；Debug 输出字段。Default 选择 v，width/precision 为 None，所有标志为 false。此 Rust 语法在原生环境测试；JavaScript 对象使用普通 JS 字段对象语义及 GoFormat.default()。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-impl-default" locale="zh-Hans" />

## 相关方法

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
