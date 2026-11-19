---
locale: zh-Hant
source_revision: 68a6c601d819a031d04a43cce835c2569811cfe3ffa0049911e2723c800f6029
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::r#mod
---
# Decimal::r#mod

回傳向零截斷除法的帶符號餘數。

```rust
fn r#mod (& self , other : & Self) -> Result < Self , DecimalError >
```

## 參數、回傳值及常規用法

other: &Decimal；Rust 回傳 Result<Decimal, DecimalError>。餘數與被除數同號，小數位數為兩個輸入中較大者；不是非負的歐幾里得取模。零除數回傳 DivideByZero。輸入僅被借用。mod 是 Rust 關鍵字，因此使用 r#mod；JS 使用 .mod(other)。兩種名稱呼叫相同核心行為。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-r-mod" locale="zh-Hant" />

## 相關方法

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
