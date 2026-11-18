---
locale: zh-Hant
source_revision: fa1a3656d76ca2759cc49aed5b71589136f0ed7e84a57f811be7062ea9aa8860
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::into_coefficient
---
# Decimal::into_coefficient

移動出未縮放的整數係數。

```rust
fn into_coefficient (self) -> BigInt
```

## 參數、回傳值及常規用法

無參數；Rust 回傳 BigInt，JS 回傳 bigint。丟棄小數位數和初始化狀態，因此 1.2300 得到 12300。預設零得到 0。Decimal 被消耗，不能再次讀取；bridge 中需要不消耗的快照時使用 coefficient。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-into-coefficient" locale="zh-Hant" />

## 瀏覽器邊界

遵循上述所有權規則。移動後的物件再次存取會回傳 moved 錯誤；free 可重複呼叫。類型或別名檢查在移動前執行，進入 Rust 後的錯誤不會恢復已移動的接收者。[資源限制](../resource-policy) 獨立於程式庫錯誤；冪運算檢查中間值，對齊運算檢查係數增長。保守檢查可能拒絕最終結果很小的運算。

## 相關方法

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
