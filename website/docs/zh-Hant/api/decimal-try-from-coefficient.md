---
locale: zh-Hant
source_revision: 34aa17297c2ce6b5b73e997c910f57874da73c70f9cf52cd3fadd2ab710a529d
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::try_from_coefficient
---
# Decimal::try_from_coefficient

透過擁有所有權的係數和小數位數建構已初始化值。

```rust
fn try_from_coefficient (coefficient : BigInt , scale : usize) -> Result < Self , DecimalError >
```

## 參數、回傳值及常規用法

coefficient: BigInt；scale: usize（非負小數位數）。保留末尾零和零值的精度，不使用文字解析器的正規化。Rust 在超過 isize::MAX 時回傳 Overflow。瀏覽器使用精確 bigint 和 wasm32 usize 範圍內的整數 number；超過 i32::MAX 的 scale 保留 Overflow，其他超過 1000 的 scale 觸及 playground 資源限制。JS bigint 不可變，因此不是被移動的包裝物件。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-try-from-coefficient" locale="zh-Hant" />

## 瀏覽器邊界

遵循上述所有權規則。移動後的物件再次存取會回傳 moved 錯誤；free 可重複呼叫。類型或別名檢查在移動前執行，進入 Rust 後的錯誤不會恢復已移動的接收者。[資源限制](../resource-policy) 獨立於程式庫錯誤；冪運算檢查中間值，對齊運算檢查係數增長。保守檢查可能拒絕最終結果很小的運算。

## 相關方法

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
