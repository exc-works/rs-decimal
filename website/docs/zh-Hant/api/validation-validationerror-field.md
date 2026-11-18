---
locale: zh-Hant
source_revision: 9d0e7a5134f00bf353febb8beb1a65cb4dacb67d9c7e5b41dbb343b7baa674b0
translation_provenance: zh-Hans-terminology-reviewed
api_id: "validation::ValidationError.field"
---
# validation::ValidationError.field

查看失敗驗證記錄。

```rust
field: String
```

## 參數、回傳值及常規用法

公開的自有 String/string，儲存呼叫者提供的欄位名。不是屬性查找路徑或 HTML 片段。翻譯時按字面插入花括號。修改錯誤記錄不會改變規則或小數。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror-field" locale="zh-Hant" />

## 瀏覽器邊界

validation feature 已啟用。單個字串最多 8192 個 UTF-8 位元組；規則清單、錯誤清單和訊息映射最多 64 項，JSON 傳輸最多 65536 位元組。小數規則參數遵循 [十進位資源限制](../resource-policy)。翻譯前保守估計預留位置展開後的總文字，超過 65536 位元組時拒絕；可能拒絕實際剛好容納的結果。自訂類別和欄位是資料，不執行程式碼。驗證失敗與組態錯誤、bridge 資源錯誤分開報告。

## 相關方法

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
