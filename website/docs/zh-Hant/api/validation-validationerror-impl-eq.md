---
locale: zh-Hant
source_revision: 4b5fc87cb7547a8cb739a71f9c75b83ad78ac1a07faee17459dcd903e68f40c4
translation_provenance: zh-Hans-terminology-reviewed
api_id: "validation::ValidationError::impl Eq"
---
# validation::ValidationError::impl Eq

使用 Rust Eq 實現。

```rust
impl Eq for validation::ValidationError
```

## 參數、回傳值及常規用法

ValidationError 相等同時比較 field、tag 和原始 parameter，不是只比較翻譯後的訊息。這是原生 Rust trait 語法，不是瀏覽器方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror-impl-eq" locale="zh-Hant" />

## 瀏覽器邊界

validation feature 已啟用。單個字串最多 8192 個 UTF-8 位元組；規則清單、錯誤清單和訊息映射最多 64 項，JSON 傳輸最多 65536 位元組。小數規則參數遵循 [十進位資源限制](../resource-policy)。翻譯前保守估計預留位置展開後的總文字，超過 65536 位元組時拒絕；可能拒絕實際剛好容納的結果。自訂類別和欄位是資料，不執行程式碼。驗證失敗與組態錯誤、bridge 資源錯誤分開報告。

## 相關方法

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
