---
locale: zh-Hans
source_revision: 1cd08b3844b9339373458bc008707757ca17e7772da4c888bc88a31f9efef788
api_id: Decimal::min
---
# Decimal::min

复制两个数值中较小的一个。

```rust
fn min (& self , other : & Self) -> Self
```

## 参数、返回值及注意事项

借用 other: &Decimal，返回独立的 Decimal。相等时保留第一个操作数的小数位数及初始化状态。Rust 中使用 Decimal::min(&left, &right)，避免选中会取得所有权的 Ord::min trait 方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-min" locale="zh-Hans" />

## 错误及边界

这些方法借用输入，不会消耗或改变输入。库操作本身不返回 Result。bridge 拒绝已释放、已移动或来自其他运行实例的对象，并应用 [资源限制](../resource-policy)。示例用 `value` 字段展示标量结果；Decimal 结果展示完整快照。

## 相关方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
