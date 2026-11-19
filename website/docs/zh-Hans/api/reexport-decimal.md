---
locale: zh-Hans
source_revision: 6976205af89f0a7bb675a6147933c2fe624e8c9d96703f7f354c0cb5ca52f755
api_id: reexport::Decimal
---
# reexport::Decimal

从 rs_decimal crate 根路径导入 Decimal。

```rust
pub use decimal::Decimal as Decimal
```

## 常规用法及语义

使用 rs_decimal::Decimal；这是同一条目的公开导入路径，不是额外的数值实现。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-decimal" locale="zh-Hans" />

此 Rust 语法示例由原生测试执行，不计作浏览器 trait 支持。

## 相关文档

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
