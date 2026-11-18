---
locale: zh-Hans
source_revision: 056afbd5297b8421283bf4270ff73e28ea9129becd2598b31ff7a84ee04dacd5
api_id: NullDecimal::impl Clone
---
# NullDecimal::impl Clone

在 Rust 中同时克隆有效标记及内部 Decimal。

```rust
impl Clone for NullDecimal
```

## 常规用法及语义

即使 valid 为 false，克隆也保留内部值。这是原生 Rust trait，不是实时共享引用。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-nulldecimal-impl-clone" locale="zh-Hans" />

此 Rust 语法示例由原生测试执行，不计作浏览器 trait 支持。

## 相关文档

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
