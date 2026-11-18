---
locale: zh-Hant
source_revision: 1933adb169124e0ed9d429b21c95687e4fd59240647c4b0f684ca7bc76aabd2b
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::big_int
---
# Decimal::big_int

複製精確的未縮放整數係數。

```rust
fn big_int (& self) -> BigInt
```

## 參數、回傳值及注意事項

Rust 回傳獨立的 BigInt，bridge 回傳 bigint。它包含末尾零，不是截斷小數的整數轉換。範例為 JSON 輸出把 bigint 轉成文字。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-big-int" locale="zh-Hant" />

## 錯誤及邊界

這些方法借用輸入，不會消耗或改變輸入。程式庫操作本身不回傳 Result。bridge 拒絕已釋放、已移動或來自其他執行實例的物件，並應用 [資源限制](../resource-policy)。範例用 `value` 欄位展示標量結果；Decimal 結果展示完整快照。

## 相關方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
