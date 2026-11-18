---
locale: zh-Hant
source_revision: fe8ae2df48c06b20c3ac1c7bbd224d12734d4193d035a1a96ed7e524e49fd64b
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::lt
---
# Decimal::lt

判斷目前數值是否小於另一個數值。

```rust
fn lt (& self , other : & Self) -> bool
```

## 參數、回傳值及注意事項

借用 other: &Decimal，回傳 bool/boolean。數值相等時回傳 false，即使儲存的小數位數不同。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-lt" locale="zh-Hant" />

## 錯誤及邊界

這些方法借用輸入，不會消耗或改變輸入。程式庫操作本身不回傳 Result。bridge 拒絕已釋放、已移動或來自其他執行實例的物件，並應用 [資源限制](../resource-policy)。範例用 `value` 欄位展示標量結果；Decimal 結果展示完整快照。

## 相關方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
