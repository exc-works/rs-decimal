---
locale: zh-Hans
source_revision: 1933adb169124e0ed9d429b21c95687e4fd59240647c4b0f684ca7bc76aabd2b
api_id: Decimal::big_int
---
# Decimal::big_int

复制精确的未缩放整数系数。

```rust
fn big_int (& self) -> BigInt
```

## 参数、返回值及注意事项

Rust 返回独立的 BigInt，bridge 返回 bigint。它包含末尾零，不是截断小数的整数转换。示例为 JSON 输出把 bigint 转成文本。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-big-int" locale="zh-Hans" />

## 错误及边界

这些方法借用输入，不会消耗或改变输入。库操作本身不返回 Result。bridge 拒绝已释放、已移动或来自其他运行实例的对象，并应用 [资源限制](../resource-policy)。示例用 `value` 字段展示标量结果；Decimal 结果展示完整快照。

## 相关方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
