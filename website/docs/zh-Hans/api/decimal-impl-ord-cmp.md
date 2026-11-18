---
locale: zh-Hans
source_revision: 317e1f4c19916312bb2823fc0a526f5311afada3dc44c8d3247bdd5fc0f08111
api_id: Decimal::impl Ord::cmp
---
# Decimal::impl Ord::cmp

通过 Rust Ord 按数值对 Decimal 排序。

```rust
fn cmp (& self , other : & Self) -> Ordering
```

## 常规用法及语义

Ord::cmp 返回 Ordering，不同于返回 -1/0/1 的 Decimal::cmp。表示差异不影响排序。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-ord-cmp" locale="zh-Hans" />

此 Rust 语法示例由原生测试执行，不计作浏览器 trait 支持。

## 相关文档

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
