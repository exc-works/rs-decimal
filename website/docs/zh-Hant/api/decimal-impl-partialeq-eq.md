---
locale: zh-Hant
source_revision: 1d252fa81f50f2414d333721b6e7df59a5331a4689705908696e9896f6fb4dbf
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::impl PartialEq::eq
---
# Decimal::impl PartialEq::eq

使用 Rust 運算子或 PartialEq 比較數值是否相等。

```rust
fn eq (& self , other : & Self) -> bool
```

## 常規用法及語義

eq 借用兩個運算元並回傳 bool；12.00 == 12.0。JavaScript 使用 equal()，而 === 判斷包裝物件身份。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-partialeq-eq" locale="zh-Hant" />

此 Rust 語法範例由原生測試執行，不計作瀏覽器 trait 支持。

## 相關文件

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
