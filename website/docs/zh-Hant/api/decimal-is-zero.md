---
locale: zh-Hant
source_revision: aa764a04d58bbac840fe64bca919cb70175f064a9010f9a840a1f56ffacabc1c
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::is_zero
---
# Decimal::is_zero

在後續操作前判斷數值是否為零。

```rust
fn is_zero (& self) -> bool
```

## 參數、回傳值及注意事項

回傳 bool/boolean，未初始化的預設值也回傳 true。需要區分初始化狀態時使用 is_nil()。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-is-zero" locale="zh-Hant" />

## 錯誤及邊界

這些方法借用輸入，不會消耗或改變輸入。程式庫操作本身不回傳 Result。bridge 拒絕已釋放、已移動或來自其他執行實例的物件，並應用 [資源限制](../resource-policy)。範例用 `value` 欄位展示標量結果；Decimal 結果展示完整快照。

## 相關方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
