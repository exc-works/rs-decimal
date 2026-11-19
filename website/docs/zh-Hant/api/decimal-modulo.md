---
locale: zh-Hant
source_revision: 561bf3d1118c3ce8a6c2b94160186e43d5ce2153ae6df6a545c57b9a2538ed41
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::modulo
---
# Decimal::modulo

回傳向零截斷除法的帶符號餘數。

```rust
fn modulo (& self , other : & Self) -> Result < Self , DecimalError >
```

## 參數、回傳值及常規用法

other: &Decimal；Rust 回傳 Result<Decimal, DecimalError>。餘數與被除數同號，小數位數為兩個輸入中較大者；不是非負的歐幾里得取模。零除數回傳 DivideByZero。輸入僅被借用。同時取得商和餘數請使用 quo_rem。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-modulo" locale="zh-Hant" />

## 相關方法

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
