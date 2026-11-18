---
locale: zh-Hans
source_revision: 5956431328357316a87f05c2d1543bb100b12923bb2d50b1e21ff3f96212db45
api_id: Decimal::max
---
# Decimal::max

复制两个数值中较大的一个。

```rust
fn max (& self , other : & Self) -> Self
```

## 参数、返回值及注意事项

借用 other: &Decimal，返回独立的 Decimal。相等时保留第一个操作数的小数位数及初始化状态。Rust 中使用 Decimal::max(&left, &right) 明确选择借用参数的固有方法；Ord::max 会取得所有权。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-max" locale="zh-Hans" />

## 错误及边界

这些方法借用输入，不会消耗或改变输入。库操作本身不返回 Result。bridge 拒绝已释放、已移动或来自其他运行实例的对象，并应用 [资源限制](../resource-policy)。示例用 `value` 字段展示标量结果；Decimal 结果展示完整快照。

## 相关方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
