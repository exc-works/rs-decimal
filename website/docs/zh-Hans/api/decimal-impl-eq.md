---
locale: zh-Hans
source_revision: cfbab45ed5115c58df815ab40afe368434f97862eeaecc53b795f1f752fe2e25
api_id: Decimal::impl Eq
---
# Decimal::impl Eq

在 Rust 要求完全相等关系的场景中使用 Decimal。

```rust
impl Eq for Decimal 
```

## 常规用法及语义

Eq 是不含方法的标记 trait。相等关系比较数值，忽略小数位数及初始化状态，并不意味着 JavaScript 对象身份相同。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-eq" locale="zh-Hans" />

此 Rust 语法示例由原生测试执行，不计作浏览器 trait 支持。

## 相关文档

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
