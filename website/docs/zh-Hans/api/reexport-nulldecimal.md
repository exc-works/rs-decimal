---
locale: zh-Hans
source_revision: f04757565931a8e8fa648872ee9153875f735f45247f70875bf4fa04dbb2585d
api_id: reexport::NullDecimal
---
# reexport::NullDecimal

从 rs_decimal crate 根路径导入 NullDecimal。

```rust
pub use decimal::NullDecimal as NullDecimal
```

## 常规用法及语义

使用 rs_decimal::NullDecimal；这是同一条目的公开导入路径，不是额外的数值实现。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-nulldecimal" locale="zh-Hans" />

此 Rust 语法示例由原生测试执行，不计作浏览器 trait 支持。

## 相关文档

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
