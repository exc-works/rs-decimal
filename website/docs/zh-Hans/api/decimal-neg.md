---
locale: zh-Hans
source_revision: 16b33be7c90300f9c4b236f60591e3b54c1078c3a213fb54df0f571ca8f60e09
api_id: Decimal::neg
---
# Decimal::neg

创建符号相反的独立 Decimal。

```rust
fn neg (& self) -> Self
```

## 参数、返回值及注意事项

返回 Decimal，保留小数位数及末尾零。零在数值上仍为零；即使输入为默认值，结果也会初始化。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-neg" locale="zh-Hans" />

## 错误及边界

这些方法借用输入，不会消耗或改变输入。库操作本身不返回 Result。bridge 拒绝已释放、已移动或来自其他运行实例的对象，并应用 [资源限制](../resource-policy)。示例用 `value` 字段展示标量结果；Decimal 结果展示完整快照。

## 相关方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
