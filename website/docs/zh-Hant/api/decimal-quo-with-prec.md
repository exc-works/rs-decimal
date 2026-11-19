---
locale: zh-Hant
source_revision: eb9bcc338f948ddd5ec14bfd00bb5bb38695bb5e5eba9b781fe855d89569b0a0
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::quo_with_prec
---
# Decimal::quo_with_prec

以指定精度或輸入精度相除。

```rust
fn quo_with_prec (& self , other : & Self , prec : isize , mode : RoundingMode ,) -> Result < Self , DecimalError >
```

## 參數、回傳值及常規用法

other: &Decimal；Rust 回傳 Result<Decimal, DecimalError>。prec 指定非負的小數位數；負精度在檢查除數前回傳 InvalidPrecision。零除數回傳 DivideByZero。mode 控制捨入；Unnecessary 對不精確結果回傳 RoundUnnecessary。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-quo-with-prec" locale="zh-Hant" />

## 相關方法

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
