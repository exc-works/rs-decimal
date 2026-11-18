---
locale: zh-Hans
source_revision: 3ced5d99beade676205267c80e93f90c49f974486393990035a12b18f4061c4a
api_id: Decimal::impl PartialEq
---
# Decimal::impl PartialEq

使用 Rust 运算符或 PartialEq 比较数值是否相等。

```rust
impl PartialEq for Decimal 
```

## 常规用法及语义

eq 借用两个操作数并返回 bool；12.00 == 12.0。JavaScript 使用 equal()，而 === 判断包装对象身份。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-partialeq" locale="zh-Hans" />

此 Rust 语法示例由原生测试执行，不计作浏览器 trait 支持。

## 相关文档

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
