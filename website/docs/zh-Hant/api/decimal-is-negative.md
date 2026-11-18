---
locale: zh-Hant
source_revision: 9ffad200b8eee0be119e53ec6189311fb6b54cada31774d04149bdc2b868ac4e
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::is_negative
---
# Decimal::is_negative

判斷數值是否嚴格小於零。

```rust
fn is_negative (& self) -> bool
```

## 參數、回傳值及注意事項

回傳 bool/boolean。零（包括未初始化的預設值）回傳 false。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-is-negative" locale="zh-Hant" />

## 錯誤及邊界

這些方法借用輸入，不會消耗或改變輸入。程式庫操作本身不回傳 Result。bridge 拒絕已釋放、已移動或來自其他執行實例的物件，並應用 [資源限制](../resource-policy)。範例用 `value` 欄位展示標量結果；Decimal 結果展示完整快照。

## 相關方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
