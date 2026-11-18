---
locale: zh-Hans
source_revision: 584a930ae7d128f9ab20d43eb7ffff2b455374bd06ee0e33745542254e1599b0
api_id: "RoundingMode::impl TryFrom < i32 >::try_from"
---
# RoundingMode::impl TryFrom < i32 >::try_from

通过 TryFrom 验证原始舍入模式值。

```rust
fn try_from (value : i32) -> Result < Self , Self :: Error >
```

## 常规用法与注意事项

value: i32；返回 `Result<RoundingMode, DecimalError>`。0–6 使用固定映射，其中 5 为 HalfEven。未知值返回 InvalidArgument，不 panic。关联 Error 类型为 DecimalError。此实现委托 from_raw；浏览器调用 RoundingMode.from_raw(number)，严格检查 i32 参数。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-roundingmode-impl-tryfrom-i32-try-from" locale="zh-Hans" />

## 执行方式

此条目是 Rust 原生 trait 语法，不提供同名 JavaScript 运算符或 SQLx 对象。 此示例由默认原生文档测试执行。

## 相关方法

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
