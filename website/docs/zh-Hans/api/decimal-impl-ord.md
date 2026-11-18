---
locale: zh-Hans
source_revision: fc360056fa6a06d0df76e11b3c2a65adb20ebcb5dcc14e8a0ad1eefc1447a90e
api_id: Decimal::impl Ord
---
# Decimal::impl Ord

通过 Rust Ord 按数值对 Decimal 排序。

```rust
impl Ord for Decimal 
```

## 常规用法及语义

Ord::cmp 返回 Ordering，不同于返回 -1/0/1 的 Decimal::cmp。表示差异不影响排序。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-ord" locale="zh-Hans" />

此 Rust 语法示例由原生测试执行，不计作浏览器 trait 支持。

## 相关文档

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
