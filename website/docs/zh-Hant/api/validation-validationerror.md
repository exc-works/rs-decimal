---
locale: zh-Hant
source_revision: 377152e0483ddcbebcd02f161fe043c129efdc1f915e3eb70b399c28e3d3906b
translation_provenance: zh-Hans-terminology-reviewed
api_id: "validation::ValidationError"
---
# validation::ValidationError

查看失敗驗證記錄。

```rust
pub struct ValidationError
```

## 參數、回傳值及常規用法

失敗記錄包含公開 field、tag、parameter 屬性。ValidationRule.validate 失敗時回傳，validate_decimal 回傳其清單。JS 提供可編輯欄位記錄，用於翻譯。它們不是 DecimalError 類別或異常訊息。修改錯誤記錄不會改變規則或小數。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror" locale="zh-Hant" />

## 瀏覽器邊界

validation feature 已啟用。單個字串最多 8192 個 UTF-8 位元組；規則清單、錯誤清單和訊息映射最多 64 項，JSON 傳輸最多 65536 位元組。小數規則參數遵循 [十進位資源限制](../resource-policy)。翻譯前保守估計預留位置展開後的總文字，超過 65536 位元組時拒絕；可能拒絕實際剛好容納的結果。自訂類別和欄位是資料，不執行程式碼。驗證失敗與組態錯誤、bridge 資源錯誤分開報告。

## 相關方法

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
