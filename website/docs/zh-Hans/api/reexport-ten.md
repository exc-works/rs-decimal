---
locale: zh-Hans
source_revision: 7a73d9c8a38938796f2eacbe88712430716dafd16a136172a660bfe237d37db2
api_id: reexport::ten
---
# reexport::ten

从 rs_decimal crate 根路径导入 ten。

```rust
pub use decimal::ten as ten
```

## 常规用法及语义

使用 rs_decimal::ten；这是同一条目的公开导入路径，不是额外的数值实现。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-ten" locale="zh-Hans" />

此 Rust 语法示例由原生测试执行，不计作浏览器 trait 支持。

## 相关文档

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
