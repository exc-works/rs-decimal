---
locale: zh-Hant
source_revision: 59688cddd572412f36dd0f36cc15c34efbc891dab38cd57b614763347d469369
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::is_nil
---
# Decimal::is_nil

判斷 Decimal 是否為未初始化的預設值。

```rust
fn is_nil (& self) -> bool
```

## 參數、回傳值及注意事項

回傳 bool/boolean。預設值未初始化，建構得到的零已初始化，但兩者數值相等。這也不是 NullDecimal 的有效性標記。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-is-nil" locale="zh-Hant" />

## 錯誤及邊界

這些方法借用輸入，不會消耗或改變輸入。程式庫操作本身不回傳 Result。bridge 拒絕已釋放、已移動或來自其他執行實例的物件，並應用 [資源限制](../resource-policy)。範例用 `value` 欄位展示標量結果；Decimal 結果展示完整快照。

## 相關方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
