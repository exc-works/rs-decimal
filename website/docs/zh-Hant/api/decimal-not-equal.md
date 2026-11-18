---
locale: zh-Hant
source_revision: 7f94821cf570d2860956d25bb69a5d1cdbe5a253fac837b02903b87236f85c19
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::not_equal
---
# Decimal::not_equal

判斷兩個 Decimal 的數值是否不同。

```rust
fn not_equal (& self , other : & Self) -> bool
```

## 參數、回傳值及注意事項

借用 other: &Decimal，回傳 bool/boolean。這是 equal() 的反向判斷，不是對係數及小數位數快照的比較。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-not-equal" locale="zh-Hant" />

## 錯誤及邊界

這些方法借用輸入，不會消耗或改變輸入。程式庫操作本身不回傳 Result。bridge 拒絕已釋放、已移動或來自其他執行實例的物件，並應用 [資源限制](../resource-policy)。範例用 `value` 欄位展示標量結果；Decimal 結果展示完整快照。

## 相關方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
