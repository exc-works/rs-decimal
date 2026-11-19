---
locale: zh-Hans
source_revision: 53c6a8acf9ea21b47dcf7e64e91f78d378746d55bbda43af84954ef21da2900f
api_id: "Decimal::impl fmt :: Display::fmt"
---
# Decimal::impl fmt :: Display::fmt

通过 Rust 格式化语法输出 Decimal。

```rust
fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result
```

## 常规用法及语义

使用 format!("{}", value)、println! 或 to_string()；fmt::Formatter 由格式化机制提供。默认格式会去掉小数末尾零。其他格式标志由格式化分组说明；JavaScript 使用 string()/string_with_trailing_zeros()。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-fmt-display-fmt" locale="zh-Hans" />

此 Rust 语法示例由原生测试执行，不计作浏览器 trait 支持。

## 相关文档

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
