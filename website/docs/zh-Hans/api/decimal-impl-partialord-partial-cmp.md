---
locale: zh-Hans
source_revision: 1a22225e09dfc3b3b1f7504680c33552c2bd9a622eca096fdf946a5ba9c5cfaa
api_id: Decimal::impl PartialOrd::partial_cmp
---
# Decimal::impl PartialOrd::partial_cmp

通过 Rust 的偏序比较 Decimal。

```rust
fn partial_cmp (& self , other : & Self) -> Option < Ordering >
```

## 常规用法及语义

所有 Decimal 值都可比较，因此 partial_cmp 返回 Some(Ordering)。默认零可与构造的零比较，小数位数不同不会产生 None。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-partialord-partial-cmp" locale="zh-Hans" />

此 Rust 语法示例由原生测试执行，不计作浏览器 trait 支持。

## 相关文档

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
