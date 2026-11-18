---
locale: zh-Hant
source_revision: 4d2afeddcd523e94b9e9276fe6f8c19193bfe95254c1ef9ded67b301a566152b
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::min_ref
---
# Decimal::min_ref

按數值比較選擇原始運算元。

```rust
fn min_ref < 'a > (& 'a self , other : & 'a Self) -> & 'a Self
```

## 參數、回傳值及常規用法

other: &Decimal；Rust 回傳與輸入生命週期關聯的共享參照。數值相等時選擇 self，保留精度和初始化狀態。bridge 回傳對應的原始物件，不複製。移動或釋放此物件會使它的所有 JS 別名失效；JS 不能強制 Rust 借用生命週期。需要獨立擁有的結果請使用 max/min。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-min-ref" locale="zh-Hant" />

## 瀏覽器邊界

遵循上述所有權規則。移動後的物件再次存取會回傳 moved 錯誤；free 可重複呼叫。類型或別名檢查在移動前執行，進入 Rust 後的錯誤不會恢復已移動的接收者。[資源限制](../resource-policy) 獨立於程式庫錯誤；冪運算檢查中間值，對齊運算檢查係數增長。保守檢查可能拒絕最終結果很小的運算。

## 相關方法

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
