---
locale: zh-Hans
source_revision: 19a39638a4734aeba49de6f5af4a9f79e89b842f7b78ba2cf7aeb511f0ebcd6c
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

## 错误及边界

这些方法借用输入，不会消耗或改变输入。库操作本身不返回 Result。bridge 拒绝已释放、已移动或来自其他运行实例的对象，并应用 [资源限制](../resource-policy)。示例用 `value` 字段展示标量结果；Decimal 结果展示完整快照。

## 相关方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
