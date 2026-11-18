---
locale: zh-Hant
source_revision: 6d86e4a4d53be90de2e3c79ca0231980d419a71194ef67f300c06ca432f017c2
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::equal
---
# Decimal::equal

判斷兩個 Decimal 的數值是否相同。

```rust
fn equal (& self , other : & Self) -> bool
```

## 參數、回傳值及注意事項

借用 other: &Decimal，回傳 bool/boolean，忽略小數位數及初始化狀態。JavaScript 的 === 比較包裝物件身份；數值相等應呼叫 equal()。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-equal" locale="zh-Hant" />

## 錯誤及邊界

這些方法借用輸入，不會消耗或改變輸入。程式庫操作本身不回傳 Result。bridge 拒絕已釋放、已移動或來自其他執行實例的物件，並應用 [資源限制](../resource-policy)。範例用 `value` 欄位展示標量結果；Decimal 結果展示完整快照。

## 相關方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
