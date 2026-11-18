---
locale: zh-Hans
source_revision: 61f8c035d4ee9a0675e175237d6b888b80543e4b200bab686287d3d96e9f1847
api_id: "Decimal::impl From < u8 >"
---
# Decimal::impl From < u8 >

通过 Rust From 精确转换 u8。

```rust
impl From<u8> for Decimal
```

## 常规用法与注意事项

消耗 u8 并返回已初始化、scale 为零的 Decimal。不经过浮点数，不舍入；对源整数类型转换不会失败。零也会初始化，与 Decimal::default() 不同。源范围由 Rust 整数类型决定；isize/usize 宽度依赖原生目标。浏览器使用明确的具名构造函数或 ConfigValue 有符号/无符号变体，Rust From/Into 语法仅用于原生环境。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-from-u8" locale="zh-Hans" />

## 执行方式

此条目是 Rust 原生 trait 语法，不提供同名 JavaScript 运算符或 SQLx 对象。 此示例由默认原生文档测试执行。

## 相关方法

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
