---
locale: zh-Hans
source_revision: 7f94821cf570d2860956d25bb69a5d1cdbe5a253fac837b02903b87236f85c19
api_id: Decimal::not_equal
---
# Decimal::not_equal

判断两个 Decimal 的数值是否不同。

```rust
fn not_equal (& self , other : & Self) -> bool
```

## 参数、返回值及注意事项

借用 other: &Decimal，返回 bool/boolean。这是 equal() 的反向判断，不是对系数及小数位数快照的比较。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-not-equal" locale="zh-Hans" />

## 错误及边界

这些方法借用输入，不会消耗或改变输入。库操作本身不返回 Result。bridge 拒绝已释放、已移动或来自其他运行实例的对象，并应用 [资源限制](../resource-policy)。示例用 `value` 字段展示标量结果；Decimal 结果展示完整快照。

## 相关方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
