---
locale: zh-Hant
source_revision: cd0b0af7153d96e3d2f3f7126c5b673c301639179f9c4225b3dd81e2a74678a4
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::add_owned
---
# Decimal::add_owned

消耗左運算元進行精確運算。

```rust
fn add_owned (self , other : & Self) -> Self
```

## 參數、回傳值及常規用法

other: &Decimal；回傳小數位數為兩個輸入中較大者的已初始化 Decimal。接收者被移動，other 仍可使用。可能重複使用儲存，但增長仍會分配。bridge 在移動前拒絕同一物件同時作為接收者和 other。進入 Rust 後，即使資源錯誤也會消耗接收者。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-add-owned" locale="zh-Hant" />

## 瀏覽器邊界

遵循上述所有權規則。移動後的物件再次存取會回傳 moved 錯誤；free 可重複呼叫。類型或別名檢查在移動前執行，進入 Rust 後的錯誤不會恢復已移動的接收者。[資源限制](../resource-policy) 獨立於程式庫錯誤；冪運算檢查中間值，對齊運算檢查係數增長。保守檢查可能拒絕最終結果很小的運算。

## 相關方法

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
