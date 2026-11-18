---
locale: zh-Hant
source_revision: ab458b72e36df078d37dd6de4be21a55e10d1e6c955c47b4abcbe87cbbdd8e23
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

## 瀏覽器邊界

遵循上述所有權規則。移動後的物件再次存取會回傳 moved 錯誤；free 可重複呼叫。類型或別名檢查在移動前執行，進入 Rust 後的錯誤不會恢復已移動的接收者。[資源限制](../resource-policy) 獨立於程式庫錯誤；冪運算檢查中間值，對齊運算檢查係數增長。保守檢查可能拒絕最終結果很小的運算。

## 相關方法

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
