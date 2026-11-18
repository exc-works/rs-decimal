---
locale: zh-Hant
source_revision: c63b4582e5ea4e7eac387ce70eb29aed40ab3d4cb5d5c76ae95a6cd8fa1f8f69
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::scale
---
# Decimal::scale

讀取儲存的非負小數位數。

```rust
fn scale (& self) -> usize
```

## 參數、回傳值及注意事項

回傳 usize，在 Playground 精度限制內映射為精確的 number；數值與 precision() 相同。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-scale" locale="zh-Hant" />

## 錯誤及邊界

這些方法借用輸入，不會消耗或改變輸入。程式庫操作本身不回傳 Result。bridge 拒絕已釋放、已移動或來自其他執行實例的物件，並應用 [資源限制](../resource-policy)。範例用 `value` 欄位展示標量結果；Decimal 結果展示完整快照。

## 相關方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
