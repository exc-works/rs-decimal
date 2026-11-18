---
locale: zh-Hans
source_revision: 1971b01eebfee126fa34311f177b8627c3bfa706a598c61defac4fe50b57c54d
api_id: "Decimal::impl Add < & Decimal >::add"
---
# Decimal::impl Add < & Decimal >::add

使用借用式 Rust Add 语法。

```rust
fn add (self , other : & Decimal) -> Decimal
```

## 常规用法与注意事项

为 &Decimal 实现，操作数仍可使用。精确结果使用较大的操作数 scale，结果已初始化。关联 Output 为 Decimal。允许负结果，不应用 safe_add/safe_sub 的非负结果要求。trait 委托同一核心具名方法。浏览器不重载 JavaScript 运算符，应调用 bridge 的 add/sub/neg。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-add-decimal-add" locale="zh-Hans" />

## 执行方式

此条目是 Rust 原生 trait 语法，不提供同名 JavaScript 运算符或 SQLx 对象。 此示例由默认原生文档测试执行。

## 相关方法

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
