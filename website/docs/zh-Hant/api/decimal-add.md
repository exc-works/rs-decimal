---
locale: zh-Hant
source_revision: 1edc6ba7e191babd98a0717864d8696548b2086ee97162b567ea1d847a357dad
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::add
---
# Decimal::add

精確相加。

```rust
fn add (& self , other : & Self) -> Self
```

## 參數、回傳值及常規用法

other: &Decimal；回傳小數位數為兩個輸入中較大者的新 Decimal。對齊是精確的，不丟棄數字。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-add" locale="zh-Hant" />

## 相關方法

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
