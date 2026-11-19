---
locale: zh-Hant
source_revision: 10c5c67f0eca13950935316848dbaf7b4f6fbe37a761aaabb0a34ec3fe68d1c2
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::quo_rem
---
# Decimal::quo_rem

回傳整數商和帶符號的 Decimal 餘數。

```rust
fn quo_rem (& self , other : & Self) -> Result < (Self , Self) , DecimalError >
```

## 參數、回傳值及常規用法

other: &Decimal；Rust 回傳 Result<(Decimal, Decimal), DecimalError>，映射為 [quotient, remainder]。商向零截斷且 scale 為零。餘數與被除數同號，小數位數為兩個輸入中較大者，滿足 self = quotient * other + remainder。每個結果都是獨立且已初始化的物件；輸入仍可使用。零除數在對齊前回傳 DivideByZero。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-quo-rem" locale="zh-Hant" />

## 相關方法

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
