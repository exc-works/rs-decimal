---
locale: zh-Hans
source_revision: 8e2ccd4d5de92023ccad23e3b1ce732438fad5dab0ed6652057ad04ade8ca4a5
api_id: reexport::hundred
---
# reexport::hundred

从 rs_decimal crate 根路径导入 hundred。

```rust
pub use decimal::hundred as hundred
```

## 常规用法及语义

使用 rs_decimal::hundred；这是同一条目的公开导入路径，不是额外的数值实现。JavaScript 没有 Rust use 语法，对应行为见 bridge 文档。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-hundred" locale="zh-Hans" />

此 Rust 语法示例由原生测试执行，不计作浏览器 trait 支持。

## 相关文档

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
