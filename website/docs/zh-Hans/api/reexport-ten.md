---
locale: zh-Hans
source_revision: 76d9cb7e766229b955c894271a5c238caac503dc592850e7b5cbe4a6a3eb33d3
api_id: reexport::ten
---
# reexport::ten

从 rs_decimal crate 根路径导入 ten。

```rust
pub use decimal::ten as ten
```

## 常规用法及语义

使用 rs_decimal::ten；这是同一条目的公开导入路径，不是额外的数值实现。JavaScript 没有 Rust use 语法，对应行为见 bridge 文档。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-ten" locale="zh-Hans" />

此 Rust 语法示例由原生测试执行，不计作浏览器 trait 支持。

## 相关文档

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
