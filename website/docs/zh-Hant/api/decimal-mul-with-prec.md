---
locale: zh-Hant
source_revision: b2363400afff1e4c740938a16816a148faaff0faef10b304c45a776ef2dcc1ee
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::mul_with_prec
---
# Decimal::mul_with_prec

按明確的輸出精度相乘。

```rust
fn mul_with_prec (& self , other : & Self , prec : isize , mode : RoundingMode ,) -> Result < Self , DecimalError >
```

## 參數、回傳值及常規用法

other: &Decimal。結果保留 prec 個小數位；prec 必須非負，否則回傳 InvalidPrecision。顯式 mode 控制捨入；Unnecessary 在必須丟棄非零數字時回傳 RoundUnnecessary。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-mul-with-prec" locale="zh-Hant" />

## 相關方法

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
