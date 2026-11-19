---
locale: zh-Hans
source_revision: 6a03ad3f2e6f0755e57eb89608d0891566bd0e9c4b875afd8935ad4e0e27dd6d
api_id: GoFormat::impl Default::default
---
# GoFormat::impl Default::default

使用 Rust Default::default 实现。

```rust
fn default () -> Self
```

## 参数、返回值及常规用法

GoFormat 是小型 Copy 字段结构体。Clone/Copy 保留选项；Debug 输出字段。Default 选择 v，width/precision 为 None，所有标志为 false。此 Rust 语法在原生环境测试；JavaScript 对象使用普通 JS 字段对象语义及 GoFormat.default()。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-impl-default-default" locale="zh-Hans" />

## 相关方法

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
