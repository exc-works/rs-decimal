---
locale: zh-Hant
source_revision: 74fe800675809309f5a6802d09df41ce125ec348c7dc823b0a9a763856b0bb52
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::mul_exact
---
# Decimal::mul_exact

按精確方式相乘。

```rust
fn mul_exact (& self , other : & Self) -> Result < Self , DecimalError >
```

## 參數、回傳值及常規用法

other: &Decimal。結果小數位數為兩個輸入之和，保留乘積的所有數字。核心精度溢出回傳 Overflow。沒有捨入模式參數。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-mul-exact" locale="zh-Hant" />

## 相關方法

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
