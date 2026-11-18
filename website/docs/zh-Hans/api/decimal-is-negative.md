---
locale: zh-Hans
source_revision: 9ffad200b8eee0be119e53ec6189311fb6b54cada31774d04149bdc2b868ac4e
api_id: Decimal::is_negative
---
# Decimal::is_negative

判断数值是否严格小于零。

```rust
fn is_negative (& self) -> bool
```

## 参数、返回值及注意事项

返回 bool/boolean。零（包括未初始化的默认值）返回 false。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-is-negative" locale="zh-Hans" />

## 错误及边界

这些方法借用输入，不会消耗或改变输入。库操作本身不返回 Result。bridge 拒绝已释放、已移动或来自其他运行实例的对象，并应用 [资源限制](../resource-policy)。示例用 `value` 字段展示标量结果；Decimal 结果展示完整快照。

## 相关方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
