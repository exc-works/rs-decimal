---
locale: zh-Hans
source_revision: 24d7fa4a2221c5f03dbc4893af4fe9b8c3b42496692a1b9ed6c8a03465745caf
api_id: reexport::max
---
# reexport::max

从 rs_decimal crate 根路径导入 max。

```rust
pub use decimal::max as max
```

## 常规用法及语义

使用 rs_decimal::max；这是同一条目的公开导入路径，不是额外的数值实现。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-max" locale="zh-Hans" />

此 Rust 语法示例由原生测试执行，不计作浏览器 trait 支持。

## 相关文档

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
