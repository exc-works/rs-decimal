---
locale: zh-Hant
source_revision: adde9dcc9ce010498ad91d2410621453cfaa363aae0ee4c3d4598482bb6eead0
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::mul
---
# Decimal::mul

按明確的輸出精度相乘。

```rust
fn mul (& self , other : & Self , mode : RoundingMode) -> Result < Self , DecimalError >
```

## 參數、回傳值及常規用法

other: &Decimal。結果小數位數為兩個輸入中較大者，而非二者之和。保留乘積全部數字請使用 mul_exact。顯式 mode 控制捨入；Unnecessary 在必須丟棄非零數字時回傳 RoundUnnecessary。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-mul" locale="zh-Hant" />

## 相關方法

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
