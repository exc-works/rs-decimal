---
locale: zh-Hans
source_revision: b0ee5cc601aab9fac963006493ebe6b998c42307e354d93807217d424adc483c
api_id: "Decimal::impl From < i8 >"
---
# Decimal::impl From < i8 >

通过 Rust From 精确转换 i8。

```rust
impl From<i8> for Decimal
```

## 常规用法与注意事项

消耗 i8 并返回已初始化、scale 为零的 Decimal。不经过浮点数，不舍入；对源整数类型转换不会失败。零也会初始化，与 Decimal::default() 不同。源范围由 Rust 整数类型决定；isize/usize 宽度依赖原生目标。浏览器使用明确的具名构造函数或 ConfigValue 有符号/无符号变体，Rust From/Into 语法仅用于原生环境。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-from-i8" locale="zh-Hans" />

## 执行方式

此条目是 Rust 原生 trait 语法，不提供同名 JavaScript 运算符或 SQLx 对象。 此示例由默认原生文档测试执行。

## 相关方法

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
