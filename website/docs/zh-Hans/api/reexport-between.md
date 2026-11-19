---
locale: zh-Hans
source_revision: 3300205234593fd9e0974d13d203dba364588cd233c7dd62f42452687449a8f7
api_id: reexport::between
---
# reexport::between

从 rs_decimal crate 根路径导入 between。

```rust
pub use decimal::between as between
```

## 常规用法及语义

使用 rs_decimal::between；这是同一条目的公开导入路径，不是额外的数值实现。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-between" locale="zh-Hans" />

此 Rust 语法示例由原生测试执行，不计作浏览器 trait 支持。

## 相关文档

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
