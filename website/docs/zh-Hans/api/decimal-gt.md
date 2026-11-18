---
locale: zh-Hans
source_revision: 73008a8984515f08334f6194e58d2ac82c8d4e900805ae3c45e10f94eb7747b8
api_id: Decimal::gt
---
# Decimal::gt

判断当前数值是否大于另一个数值。

```rust
fn gt (& self , other : & Self) -> bool
```

## 参数、返回值及注意事项

借用 other: &Decimal，返回 bool/boolean。数值相等时返回 false，即使保存的小数位数不同。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-gt" locale="zh-Hans" />

## 错误及边界

这些方法借用输入，不会消耗或改变输入。库操作本身不返回 Result。bridge 拒绝已释放、已移动或来自其他运行实例的对象，并应用 [资源限制](../resource-policy)。示例用 `value` 字段展示标量结果；Decimal 结果展示完整快照。

## 相关方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
