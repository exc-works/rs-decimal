---
locale: zh-Hans
source_revision: aa32a5479f7baf77d635cc3726288c9b7c2cfa292167bcd9b91265a058234a8b
api_id: Decimal::abs
---
# Decimal::abs

创建保存绝对值的独立 Decimal。

```rust
fn abs (& self) -> Self
```

## 参数、返回值及注意事项

返回 Decimal，保留小数位数及末尾零。即使输入为默认值，结果也会初始化；输入保持不变。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-abs" locale="zh-Hans" />

## 错误及边界

这些方法借用输入，不会消耗或改变输入。库操作本身不返回 Result。bridge 拒绝已释放、已移动或来自其他运行实例的对象，并应用 [资源限制](../resource-policy)。示例用 `value` 字段展示标量结果；Decimal 结果展示完整快照。

## 相关方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
