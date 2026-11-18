---
locale: zh-Hant
source_revision: f56b3cc9eb8667e7cf067d7d1256d88fafb84d3cb5b89748b59c7df428bab4cf
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::into_rescaled
---
# Decimal::into_rescaled

消耗 Decimal，同時改變小數位數。

```rust
fn into_rescaled (self , prec : isize , mode : RoundingMode) -> Result < Self , DecimalError >
```

## 參數、回傳值及常規用法

prec: isize；mode: RoundingMode。與 rescale 一樣，捨入或追加零，並始終初始化結果。即使 Rust 回傳 InvalidPrecision、RoundUnnecessary 或瀏覽器資源錯誤，接收者也已移動。JS 類型或範圍不合法會在進入 Rust 前被拒絕，不消耗接收者。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-into-rescaled" locale="zh-Hant" />

## 瀏覽器邊界

遵循上述所有權規則。移動後的物件再次存取會回傳 moved 錯誤；free 可重複呼叫。類型或別名檢查在移動前執行，進入 Rust 後的錯誤不會恢復已移動的接收者。[資源限制](../resource-policy) 獨立於程式庫錯誤；冪運算檢查中間值，對齊運算檢查係數增長。保守檢查可能拒絕最終結果很小的運算。

## 相關方法

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
