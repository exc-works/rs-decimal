---
locale: zh-Hant
source_revision: 0d6262b2eb7c6955f187e4d63b88d16a39d035f514ca55c738d74b10a8d69203
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::quo
---
# Decimal::quo

以指定精度或輸入精度相除。

```rust
fn quo (& self , other : & Self , mode : RoundingMode) -> Result < Self , DecimalError >
```

## 參數、回傳值及常規用法

other: &Decimal；Rust 回傳 Result<Decimal, DecimalError>。輸出小數位數為兩個輸入中較大者，因此兩個整數輸入也產生整數：1 / 3 使用 Down 得到 0，使用 Up 則得到 1。需要小數結果請使用 quo_with_prec。零除數回傳 DivideByZero。mode 控制捨入；Unnecessary 對不精確結果回傳 RoundUnnecessary。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-quo" locale="zh-Hant" />

## 相關方法

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
