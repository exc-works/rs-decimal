---
locale: zh-Hant
source_revision: 4f9cc4e1f27490d354a3bbc2ca44814e0fd0d846ebf81964d518e23cc81067c0
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

## 瀏覽器邊界

遵循上述所有權規則。移動後的物件再次存取會回傳 moved 錯誤；free 可重複呼叫。類型或別名檢查在移動前執行，進入 Rust 後的錯誤不會恢復已移動的接收者。[資源限制](../resource-policy) 獨立於程式庫錯誤；冪運算檢查中間值，對齊運算檢查係數增長。保守檢查可能拒絕最終結果很小的運算。

## 相關方法

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
