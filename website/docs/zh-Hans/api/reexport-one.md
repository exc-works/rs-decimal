---
locale: zh-Hans
source_revision: f5ea64d7e7a6135acc2f55f3a40411c6d9222e20ac49981dab203cb401fa6651
api_id: reexport::one
---
# reexport::one

从 rs_decimal crate 根路径导入 one。

```rust
pub use decimal::one as one
```

## 常规用法及语义

使用 rs_decimal::one；这是同一条目的公开导入路径，不是额外的数值实现。JavaScript 没有 Rust use 语法，对应行为见 bridge 文档。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-one" locale="zh-Hans" />

此 Rust 语法示例由原生测试执行，不计作浏览器 trait 支持。

## 相关文档

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
