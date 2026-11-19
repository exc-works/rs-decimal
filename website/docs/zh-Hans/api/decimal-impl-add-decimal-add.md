---
locale: zh-Hans
source_revision: 25c9cc1e70c5db48b15f2e6457931c51bc64818138c0422f347742831125e26b
api_id: "Decimal::impl Add < & Decimal >::add"
---
# Decimal::impl Add < & Decimal >::add

使用借用式 Rust Add 语法。

```rust
fn add (self , other : & Decimal) -> Decimal
```

## 常规用法与注意事项

为 &Decimal 实现，操作数仍可使用。精确结果使用较大的操作数 scale，结果已初始化。关联 Output 为 Decimal。允许负结果，不应用 safe_add/safe_sub 的非负结果要求。trait 委托同一核心具名方法。浏览器不重载 JavaScript 运算符，应调用 add/sub/neg。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-add-decimal-add" locale="zh-Hans" />

## 执行方式

此条目是 Rust 原生 trait 语法，不提供同名 JavaScript 运算符或 SQLx 对象。 此示例由默认原生文档测试执行。

## 相关方法

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
