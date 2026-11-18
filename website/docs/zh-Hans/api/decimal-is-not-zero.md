---
locale: zh-Hans
source_revision: aaa15879504e10287ba605474fa0c07c65d1d6607caa2a6ac35b407fe6cd46e4
api_id: Decimal::is_not_zero
---
# Decimal::is_not_zero

判断数值是否非零。

```rust
fn is_not_zero (& self) -> bool
```

## 参数、返回值及注意事项

返回 bool/boolean，与 is_zero() 相反；未初始化的默认值返回 false。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-is-not-zero" locale="zh-Hans" />

## 错误及边界

这些方法借用输入，不会消耗或改变输入。库操作本身不返回 Result。bridge 拒绝已释放、已移动或来自其他运行实例的对象，并应用 [资源限制](../resource-policy)。示例用 `value` 字段展示标量结果；Decimal 结果展示完整快照。

## 相关方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
