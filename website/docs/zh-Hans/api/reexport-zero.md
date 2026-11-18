---
locale: zh-Hans
source_revision: 099c3533a52ede06adc92af14955d2d2a20dadf66c185c385b9eda728b52b6ba
api_id: reexport::zero
---
# reexport::zero

从 rs_decimal crate 根路径导入 zero。

```rust
pub use decimal::zero as zero
```

## 常规用法及语义

使用 rs_decimal::zero；这是同一条目的公开导入路径，不是额外的数值实现。JavaScript 没有 Rust use 语法，对应行为见 bridge 文档。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-zero" locale="zh-Hans" />

此 Rust 语法示例由原生测试执行，不计作浏览器 trait 支持。

## 相关文档

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
