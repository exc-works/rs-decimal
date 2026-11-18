---
locale: zh-Hans
source_revision: ecdf018f489484305dd62f934b29d0a6759b6939d7abb56d9717dd4b72e36899
api_id: Decimal::gte
---
# Decimal::gte

判断当前数值是否大于或等于另一个数值。

```rust
fn gte (& self , other : & Self) -> bool
```

## 参数、返回值及注意事项

借用 other: &Decimal，返回 bool/boolean。相等按数值判断，与小数位数及初始化状态无关。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-gte" locale="zh-Hans" />

## 错误及边界

这些方法借用输入，不会消耗或改变输入。库操作本身不返回 Result。bridge 拒绝已释放、已移动或来自其他运行实例的对象，并应用 [资源限制](../resource-policy)。示例用 `value` 字段展示标量结果；Decimal 结果展示完整快照。

## 相关方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
