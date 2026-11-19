---
locale: zh-Hans
source_revision: 098d219e5e0932c9734847caf4da4f02238ac75ca2e27400d9527165d61283dd
api_id: Decimal::cmp
---
# Decimal::cmp

用三态结果比较两个数值。

```rust
fn cmp (& self , other : & Self) -> i32
```

## 参数、返回值及注意事项

借用 other: &Decimal（JavaScript 中为 Decimal）。返回 i32/number：-1、0 或 1。小数位数及初始化状态不同不影响数值相等。这是固有方法，不是返回 Ordering 的 Ord::cmp。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-cmp" locale="zh-Hans" />

## 相关方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
