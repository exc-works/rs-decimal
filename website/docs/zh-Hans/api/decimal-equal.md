---
locale: zh-Hans
source_revision: 6d86e4a4d53be90de2e3c79ca0231980d419a71194ef67f300c06ca432f017c2
api_id: Decimal::equal
---
# Decimal::equal

判断两个 Decimal 的数值是否相同。

```rust
fn equal (& self , other : & Self) -> bool
```

## 参数、返回值及注意事项

借用 other: &Decimal，返回 bool/boolean，忽略小数位数及初始化状态。JavaScript 的 === 比较包装对象身份；数值相等应调用 equal()。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-equal" locale="zh-Hans" />

## 错误及边界

这些方法借用输入，不会消耗或改变输入。库操作本身不返回 Result。bridge 拒绝已释放、已移动或来自其他运行实例的对象，并应用 [资源限制](../resource-policy)。示例用 `value` 字段展示标量结果；Decimal 结果展示完整快照。

## 相关方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
