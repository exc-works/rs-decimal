---
locale: zh-Hans
source_revision: 0db8fa19358a4f2f8842ab2f8da65e8e108f7914cbdecf8f41f654ca5171b70c
api_id: reexport::max
---
# reexport::max

从 rs_decimal crate 根路径导入 max。

```rust
pub use decimal::max as max
```

## 常规用法及语义

使用 rs_decimal::max；这是同一条目的公开导入路径，不是额外的数值实现。JavaScript 没有 Rust use 语法，对应行为见 bridge 文档。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-max" locale="zh-Hans" />

此 Rust 语法示例由原生测试执行，不计作浏览器 trait 支持。

## 相关文档

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
