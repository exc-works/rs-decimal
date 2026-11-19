---
locale: zh-Hant
source_revision: 97b85000fbf7a0fb6537eae68258f927a7739647e41e96de3c0f26f460f2d35a
translation_provenance: zh-Hans-terminology-reviewed
api_id: "Decimal::impl fmt :: Display"
---
# Decimal::impl fmt :: Display

透過 Rust 格式化語法輸出 Decimal。

```rust
impl fmt :: Display for Decimal
```

## 常規用法及語義

使用 format!("{}", value)、println! 或 to_string()；fmt::Formatter 由格式化機制提供。預設格式會去掉小數末尾零。其他格式旗標由格式化分組說明；JavaScript 使用 string()/string_with_trailing_zeros()。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-fmt-display" locale="zh-Hant" />

此 Rust 語法範例由原生測試執行，不計作瀏覽器 trait 支持。

## 相關文件

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
