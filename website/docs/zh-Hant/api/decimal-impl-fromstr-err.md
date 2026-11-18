---
locale: zh-Hant
source_revision: 45b2b4aa308ba6b40046aead6dddbc34fd9a06d38be18fb996c0bc76cff67656
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::impl FromStr::Err
---
# Decimal::impl FromStr::Err

參照 Decimal 的 FromStr 實現所回傳的錯誤類型。

```rust
type Err = DecimalError
```

## 常規用法及語義

關聯類型 `<Decimal as FromStr>::Err` 就是 `DecimalError`。非法文字回傳該錯誤，不會觸發 panic。可以讀取錯誤類別來區分語法錯誤及其他失敗。這是 Rust 類型別名，不是 JavaScript 建構函數。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-fromstr-err" locale="zh-Hant" />

此 Rust 語法範例由原生測試執行，不計作瀏覽器 trait 支持。

## 相關文件

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
