---
locale: zh-Hant
source_revision: 3ced5d99beade676205267c80e93f90c49f974486393990035a12b18f4061c4a
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::impl PartialEq
---
# Decimal::impl PartialEq

使用 Rust 運算子或 PartialEq 比較數值是否相等。

```rust
impl PartialEq for Decimal
```

## 常規用法及語義

eq 借用兩個運算元並回傳 bool；12.00 == 12.0。JavaScript 使用 equal()，而 === 判斷包裝物件身份。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-partialeq" locale="zh-Hant" />

此 Rust 語法範例由原生測試執行，不計作瀏覽器 trait 支持。

## 相關文件

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
