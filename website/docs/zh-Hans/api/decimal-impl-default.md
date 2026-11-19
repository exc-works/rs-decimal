---
locale: zh-Hans
source_revision: 23ce03e01f179e79048016168712d7a3101de4dd03467bd8aefb98f6fd47b269
api_id: Decimal::impl Default
---
# Decimal::impl Default

通过 Rust Default 创建未初始化的数值零。

```rust
impl Default for Decimal
```

## 常规用法及语义

默认值在数值上与构造的零相等，但 is_nil() 为 true。JavaScript 提供 Decimal.default()；此页描述的是 Rust trait 本身。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-default" locale="zh-Hans" />

此 Rust 语法示例由原生测试执行，不计作浏览器 trait 支持。

## 相关文档

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
