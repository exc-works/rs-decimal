---
locale: zh-Hant
source_revision: 4f55aceb4528bb2f23f6df7261d576123104df5097928d86a2b9915f6c2f9824
translation_provenance: zh-Hans-terminology-reviewed
api_id: "validation::ValidationRule"
---
# validation::ValidationRule

執行精確的小數規則。

```rust
pub struct ValidationRule
```

## 參數、回傳值及常規用法

透過 parse(tag, parameter) 建立規則，再重複使用於 matches 或 validate。建構時檢查參數，瀏覽器控制代碼屬於本次執行，可以顯式釋放。規則精確比較 Decimal，不經過浮點轉換。decimal_required 檢查初始化，因此已初始化零透過。eq/ne/gt/gte/lt/lte 按數值比較，包括未初始化零。between 使用包含端點的 lower~upper，且額外要求初始化。positive/negative/nonzero 要求初始化。max_precision 檢查儲存的 scale，包括保留的末尾零；它不隱含 required。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationrule" locale="zh-Hant" />

## 瀏覽器邊界

validation feature 已啟用。單個字串最多 8192 個 UTF-8 位元組；規則清單、錯誤清單和訊息映射最多 64 項，JSON 傳輸最多 65536 位元組。小數規則參數遵循 [十進位資源限制](../resource-policy)。翻譯前保守估計預留位置展開後的總文字，超過 65536 位元組時拒絕；可能拒絕實際剛好容納的結果。自訂類別和欄位是資料，不執行程式碼。驗證失敗與組態錯誤、bridge 資源錯誤分開報告。

## 相關方法

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
