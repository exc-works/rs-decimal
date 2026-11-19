---
locale: zh-Hant
source_revision: db43c880650b6eb59e9c805ec0cdb77a4879108bb98a5d2e10ec38f65ac55f46
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::sub
---
# Decimal::sub

精確相減。

```rust
fn sub (& self , other : & Self) -> Self
```

## 參數、回傳值及常規用法

other: &Decimal；回傳小數位數為兩個輸入中較大者的新 Decimal。對齊是精確的，不丟棄數字。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-sub" locale="zh-Hant" />

## 相關方法

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
