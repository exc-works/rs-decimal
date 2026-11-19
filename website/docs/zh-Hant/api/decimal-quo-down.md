---
locale: zh-Hant
source_revision: 24cee5102dc473cb8fed8902e9829dd352863649ca89859b6c19bb3f001aa710
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::quo_down
---
# Decimal::quo_down

以指定精度或輸入精度相除。

```rust
fn quo_down (& self , other : & Self) -> Result < Self , DecimalError >
```

## 參數、回傳值及常規用法

other: &Decimal；Rust 回傳 Result<Decimal, DecimalError>。輸出小數位數為兩個輸入中較大者，因此兩個整數輸入也產生整數：1 / 3 得到 0。需要小數結果請使用 quo_with_prec。零除數回傳 DivideByZero。Down 向零截斷，不是向負無窮。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-quo-down" locale="zh-Hant" />

## 相關方法

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
