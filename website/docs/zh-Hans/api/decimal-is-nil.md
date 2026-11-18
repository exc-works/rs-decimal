---
locale: zh-Hans
source_revision: 59688cddd572412f36dd0f36cc15c34efbc891dab38cd57b614763347d469369
api_id: Decimal::is_nil
---
# Decimal::is_nil

判断 Decimal 是否为未初始化的默认值。

```rust
fn is_nil (& self) -> bool
```

## 参数、返回值及注意事项

返回 bool/boolean。默认值未初始化，构造得到的零已初始化，但两者数值相等。这也不是 NullDecimal 的有效性标记。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-is-nil" locale="zh-Hans" />

## 错误及边界

这些方法借用输入，不会消耗或改变输入。库操作本身不返回 Result。bridge 拒绝已释放、已移动或来自其他运行实例的对象，并应用 [资源限制](../resource-policy)。示例用 `value` 字段展示标量结果；Decimal 结果展示完整快照。

## 相关方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
