---
locale: zh-Hans
source_revision: aa764a04d58bbac840fe64bca919cb70175f064a9010f9a840a1f56ffacabc1c
api_id: Decimal::is_zero
---
# Decimal::is_zero

在后续操作前判断数值是否为零。

```rust
fn is_zero (& self) -> bool
```

## 参数、返回值及注意事项

返回 bool/boolean，未初始化的默认值也返回 true。需要区分初始化状态时使用 is_nil()。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-is-zero" locale="zh-Hans" />

## 错误及边界

这些方法借用输入，不会消耗或改变输入。库操作本身不返回 Result。bridge 拒绝已释放、已移动或来自其他运行实例的对象，并应用 [资源限制](../resource-policy)。示例用 `value` 字段展示标量结果；Decimal 结果展示完整快照。

## 相关方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
