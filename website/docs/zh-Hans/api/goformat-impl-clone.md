---
locale: zh-Hans
source_revision: df43bbe7e732dc8914c18a95d0b5d68a3839eb4a2b1c83d4dc811e0031827a91
api_id: GoFormat::impl Clone
---
# GoFormat::impl Clone

使用 Rust Clone 实现。

```rust
impl Clone for GoFormat
```

## 参数、返回值及常规用法

GoFormat 是小型 Copy 字段结构体。Clone/Copy 保留选项；Debug 输出字段。Default 选择 v，width/precision 为 None，所有标志为 false。此 Rust 语法在原生环境测试；bridge 对象使用普通 JS 字段对象语义及 GoFormat.default()。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-impl-clone" locale="zh-Hans" />

## 相关方法

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
