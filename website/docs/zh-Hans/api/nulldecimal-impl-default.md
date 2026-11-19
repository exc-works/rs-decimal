---
locale: zh-Hans
source_revision: c654c9726a283aeb3b59ec2a73dcd6f45d404c2677f244d97a60cf2ff73a642b
api_id: NullDecimal::impl Default
---
# NullDecimal::impl Default

通过 Rust Default 创建缺失的可空值。

```rust
impl Default for NullDecimal
```

## 常规用法及语义

有效标记与内部初始化状态均为 false。JavaScript 提供 NullDecimal.default()。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-nulldecimal-impl-default" locale="zh-Hans" />

此 Rust 语法示例由原生测试执行，不计作浏览器 trait 支持。

## 相关文档

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
