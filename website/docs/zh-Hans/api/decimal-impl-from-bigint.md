---
locale: zh-Hans
source_revision: 7fadd186172ed1d2675ccc16ae4020af71fd55a578a479ca70e021285e76f370
api_id: "Decimal::impl From < BigInt >"
---
# Decimal::impl From < BigInt >

通过 Rust From 精确转换 BigInt。

```rust
impl From < BigInt > for Decimal 
```

## 常规用法与注意事项

消耗 BigInt 并返回已初始化、scale 为零的 Decimal。不经过浮点数，不舍入；对源整数类型转换不会失败。零也会初始化，与 Decimal::default() 不同。BigInt 所有权移入小数。浏览器使用明确的具名构造函数或 ConfigValue 有符号/无符号变体，Rust From/Into 语法仅用于原生环境。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-from-bigint" locale="zh-Hans" />

## 执行方式

此条目是 Rust 原生 trait 语法，不提供同名 JavaScript 运算符或 SQLx 对象。 此示例由默认原生文档测试执行。

## 相关方法

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
