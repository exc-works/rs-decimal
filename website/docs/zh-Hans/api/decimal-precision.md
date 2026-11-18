---
locale: zh-Hans
source_revision: c8a99895a920cbd49f4dba891779055021c167a0addb6791872293807a1937f8
api_id: Decimal::precision
---
# Decimal::precision

读取数值保存的小数位数。

```rust
fn precision (& self) -> isize
```

## 参数、返回值及注意事项

返回 isize，在浏览器中为整数 number。这是小数位数，不是有效数字位数；末尾零也计入其中。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-precision" locale="zh-Hans" />

## 错误及边界

这些方法借用输入，不会消耗或改变输入。库操作本身不返回 Result。bridge 拒绝已释放、已移动或来自其他运行实例的对象，并应用 [资源限制](../resource-policy)。示例用 `value` 字段展示标量结果；Decimal 结果展示完整快照。

## 相关方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
