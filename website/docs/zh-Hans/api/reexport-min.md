---
locale: zh-Hans
source_revision: 0d0d53ee31abc5089791404b98708465e5ae1271ee54c709858dd761b9a5ca22
api_id: reexport::min
---
# reexport::min

从 rs_decimal crate 根路径导入 min。

```rust
pub use decimal::min as min
```

## 常规用法及语义

使用 rs_decimal::min；这是同一条目的公开导入路径，不是额外的数值实现。JavaScript 没有 Rust use 语法，对应行为见 bridge 文档。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-min" locale="zh-Hans" />

此 Rust 语法示例由原生测试执行，不计作浏览器 trait 支持。

## 相关文档

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
