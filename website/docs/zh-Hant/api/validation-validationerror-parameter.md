---
locale: zh-Hant
source_revision: 0e561c16634157ba3586cee74da3c90f15491d42428e944428a8ee980c8ddeb4
translation_provenance: zh-Hans-terminology-reviewed
api_id: "validation::ValidationError.parameter"
---
# validation::ValidationError.parameter

查看失敗驗證記錄。

```rust
parameter: String
```

## 參數、回傳值及常規用法

公開的自有 String/string，精確保留原始規則參數，包括空白。作為 {1} 只替換一次，不遞迴解釋模板。修改錯誤記錄不會改變規則或小數。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror-parameter" locale="zh-Hant" />

## 瀏覽器邊界

validation feature 已啟用。單個字串最多 8192 個 UTF-8 位元組；規則清單、錯誤清單和訊息映射最多 64 項，JSON 傳輸最多 65536 位元組。小數規則參數遵循 [十進位資源限制](../resource-policy)。翻譯前保守估計預留位置展開後的總文字，超過 65536 位元組時拒絕；可能拒絕實際剛好容納的結果。自訂類別和欄位是資料，不執行程式碼。驗證失敗與組態錯誤、bridge 資源錯誤分開報告。

## 相關方法

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
