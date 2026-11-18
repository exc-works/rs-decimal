---
locale: zh-Hans
source_revision: b70b6d632eef5a013491ea47ed27e9d8eea36fee70b75cd0db8637b8b6a6ccc5
api_id: Decimal::coefficient
---
# Decimal::coefficient

查看精确的未缩放整数系数。

```rust
fn coefficient (& self) -> & BigInt
```

## 参数、返回值及注意事项

Rust 借用 &BigInt；bridge 返回不可变的 bigint 快照，并非实时引用。系数包含末尾零，不是小数的整数部分。示例把 bigint 转成文本，以便 JSON 保留全部数字。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-coefficient" locale="zh-Hans" />

## 错误及边界

这些方法借用输入，不会消耗或改变输入。库操作本身不返回 Result。bridge 拒绝已释放、已移动或来自其他运行实例的对象，并应用 [资源限制](../resource-policy)。示例用 `value` 字段展示标量结果；Decimal 结果展示完整快照。

## 相关方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
