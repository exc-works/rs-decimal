---
locale: zh-Hans
source_revision: 150ae4511881de6cce3960793291dd735b00fc7aa5175f86d5957a93c70d0c13
api_id: Decimal::impl Debug
---
# Decimal::impl Debug

在 Rust 调试时查看 Decimal。

```rust
impl Debug for Decimal
```

## 常规用法及语义

派生的诊断表示包含内部字段，不是稳定的序列化格式。浏览器输出使用明确的快照。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-debug" locale="zh-Hans" />

此 Rust 语法示例由原生测试执行，不计作浏览器 trait 支持。

## 相关文档

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
