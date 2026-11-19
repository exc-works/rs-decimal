---
locale: zh-Hant
source_revision: 8f612ca2b723227e5991f08e31b92d45cb4ea058472e86db2c02e8a1c4cc372e
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::mul_down
---
# Decimal::mul_down

按明確的輸出精度相乘。

```rust
fn mul_down (& self , other : & Self) -> Self
```

## 參數、回傳值及常規用法

other: &Decimal。結果小數位數為兩個輸入中較大者，而非二者之和。保留乘積全部數字請使用 mul_exact。Down 向零截斷，包括負結果。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-mul-down" locale="zh-Hant" />

## 相關方法

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
