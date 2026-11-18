---
locale: zh-Hans
source_revision: 4b0752061c959ac5ce6e3b7af4bf7c9232d0d1cfb43869372b4caa7a5d6f49f4
api_id: NullDecimal::impl Debug
---
# NullDecimal::impl Debug

通过 Rust Debug 查看可空状态。

```rust
impl Debug for NullDecimal
```

## 常规用法及语义

诊断文本包含有效标记及存储字段，包括隐藏值。它不是稳定的传输格式，浏览器使用快照。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-nulldecimal-impl-debug" locale="zh-Hans" />

此 Rust 语法示例由原生测试执行，不计作浏览器 trait 支持。

## 相关文档

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
