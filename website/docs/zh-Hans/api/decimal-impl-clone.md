---
locale: zh-Hans
source_revision: 37fcbc194399d8059373fc9c0c61833d997ffb41871435eb1c6867b7870d1acc
api_id: Decimal::impl Clone
---
# Decimal::impl Clone

克隆 Decimal 并保留完整表示。

```rust
impl Clone for Decimal
```

## 常规用法及语义

Clone 复制系数、小数位数及初始化状态；JavaScript 使用具名 clone() 适配器，不提供 Rust trait 语法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-clone" locale="zh-Hans" />

此 Rust 语法示例由原生测试执行，不计作浏览器 trait 支持。

## 相关文档

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
