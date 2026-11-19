---
locale: zh-Hant
source_revision: b834c8dbe54adc1326f4715bae0b5a7e4a551bc4817e951238d3a76cb1960aa5
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::safe_sub
---
# Decimal::safe_sub

精確相減，要求結果非負。

```rust
fn safe_sub (& self , other : & Self) -> Result < Self , DecimalError >
```

## 參數、回傳值及常規用法

other: &Decimal；回傳小數位數為兩個輸入中較大者的新 Decimal。對齊是精確的，不丟棄數字。Rust 回傳 Result：僅要求結果非負，不要求每個輸入非負。負結果回傳 InvalidArgument，允許零結果。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-safe-sub" locale="zh-Hant" />

## 相關方法

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
