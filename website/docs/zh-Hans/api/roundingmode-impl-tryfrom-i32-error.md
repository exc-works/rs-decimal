---
locale: zh-Hans
source_revision: f397dc1220fc731959fe63aa3e5a1c46b0d68942bea1f3f5f567773d91995a1b
api_id: "RoundingMode::impl TryFrom < i32 >::Error"
---
# RoundingMode::impl TryFrom < i32 >::Error

通过 TryFrom 验证原始舍入模式值。

```rust
type Error = DecimalError
```

## 常规用法与注意事项

value: i32；返回 `Result<RoundingMode, DecimalError>`。0–6 使用固定映射，其中 5 为 HalfEven。未知值返回 InvalidArgument，不 panic。关联 Error 类型为 DecimalError。此实现委托 from_raw；浏览器调用 RoundingMode.from_raw(number)，严格检查 i32 参数。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-roundingmode-impl-tryfrom-i32-error" locale="zh-Hans" />

## 执行方式

此条目是 Rust 原生 trait 语法，不提供同名 JavaScript 运算符或 SQLx 对象。 此示例由默认原生文档测试执行。

## 相关方法

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
