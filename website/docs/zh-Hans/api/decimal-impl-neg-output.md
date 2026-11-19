---
locale: zh-Hans
source_revision: 4de2b7f049c588d3fb290c5d299ab4e7f3436be10c0a26243bffab3f3e0229cf
api_id: "Decimal::impl Neg::Output"
---
# Decimal::impl Neg::Output

使用借用式 Rust Neg 语法。

```rust
type Output = Decimal
```

## 常规用法与注意事项

为 &Decimal 实现，操作数仍可使用。一元 - 保留 scale 并初始化结果，包括默认零。trait 委托同一核心具名方法。浏览器不重载 JavaScript 运算符，应调用 add/sub/neg。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-neg-output" locale="zh-Hans" />

## 执行方式

此条目是 Rust 原生 trait 语法，不提供同名 JavaScript 运算符或 SQLx 对象。 此示例由默认原生文档测试执行。

## 相关方法

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
