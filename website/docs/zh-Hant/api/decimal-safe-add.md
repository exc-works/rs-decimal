---
locale: zh-Hant
source_revision: 197565e31e34527cbc33f567c65ff6fb5c5304500454cb2da830919e852c5158
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::safe_add
---
# Decimal::safe_add

精確相加，要求結果非負。

```rust
fn safe_add (& self , other : & Self) -> Result < Self , DecimalError >
```

## 參數、回傳值及常規用法

other: &Decimal；回傳小數位數為兩個輸入中較大者的新 Decimal。對齊是精確的，不丟棄數字。Rust 回傳 Result：僅要求結果非負，不要求每個輸入非負。負結果回傳 InvalidArgument，允許零結果。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-safe-add" locale="zh-Hant" />

## 相關方法

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
