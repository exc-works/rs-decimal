---
locale: zh-Hans
source_revision: fe8ae2df48c06b20c3ac1c7bbd224d12734d4193d035a1a96ed7e524e49fd64b
api_id: Decimal::lt
---
# Decimal::lt

判断当前数值是否小于另一个数值。

```rust
fn lt (& self , other : & Self) -> bool
```

## 参数、返回值及注意事项

借用 other: &Decimal，返回 bool/boolean。数值相等时返回 false，即使保存的小数位数不同。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-lt" locale="zh-Hans" />

## 错误及边界

这些方法借用输入，不会消耗或改变输入。库操作本身不返回 Result。bridge 拒绝已释放、已移动或来自其他运行实例的对象，并应用 [资源限制](../resource-policy)。示例用 `value` 字段展示标量结果；Decimal 结果展示完整快照。

## 相关方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
