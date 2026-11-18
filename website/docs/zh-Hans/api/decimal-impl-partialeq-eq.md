---
locale: zh-Hans
source_revision: 1d252fa81f50f2414d333721b6e7df59a5331a4689705908696e9896f6fb4dbf
api_id: Decimal::impl PartialEq::eq
---
# Decimal::impl PartialEq::eq

使用 Rust 运算符或 PartialEq 比较数值是否相等。

```rust
fn eq (& self , other : & Self) -> bool
```

## 常规用法及语义

eq 借用两个操作数并返回 bool；12.00 == 12.0。JavaScript 使用 equal()，而 === 判断包装对象身份。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-partialeq-eq" locale="zh-Hans" />

此 Rust 语法示例由原生测试执行，不计作浏览器 trait 支持。

## 相关文档

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
