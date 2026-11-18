---
locale: zh-Hans
source_revision: c94998594bf6a8005b50df98354b3549d47447f21a6664735b824dda07443935
api_id: "Decimal::impl fmt :: Display"
---
# Decimal::impl fmt :: Display

通过 Rust 格式化语法输出 Decimal。

```rust
impl fmt :: Display for Decimal 
```

## 常规用法及语义

使用 format!("{}", value)、println! 或 to_string()；fmt::Formatter 由格式化机制提供。默认格式会去掉小数末尾零。其他格式标志由格式化分组说明；bridge 使用 string()/string_with_trailing_zeros()。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-fmt-display" locale="zh-Hans" />

此 Rust 语法示例由原生测试执行，不计作浏览器 trait 支持。

## 相关文档

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
