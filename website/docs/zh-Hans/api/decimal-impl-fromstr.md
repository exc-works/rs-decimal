---
locale: zh-Hans
source_revision: 65703feb0e11af9dfe348c058c514dbf10ab13c1d2a720632926e57fee628fa9
api_id: Decimal::impl FromStr
---
# Decimal::impl FromStr

通过 Rust 的 parse 和 FromStr 解析小数文本。

```rust
impl FromStr for Decimal 
```

## 常规用法及语义

返回 Decimal 或 DecimalError。语法及零值标准化与 new_from_string 相同。浏览器调用该具名方法；Rust trait 语法仅适用于原生 Rust。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-fromstr" locale="zh-Hans" />

此 Rust 语法示例由原生测试执行，不计作浏览器 trait 支持。

## 相关文档

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
