---
locale: zh-Hant
source_revision: 92b27b2e44b2095f2d604aead4c4f01fb922094a0083fe1c3c5e2509eaa4b3d7
translation_provenance: zh-Hans-terminology-reviewed
api_id: "validation::ValidationRule::validate"
---
# validation::ValidationRule::validate

按規則驗證一個欄位。

```rust
fn validate (& self , field : & str , value : & Decimal) -> Result < () , ValidationError >
```

## 參數、回傳值及常規用法

field: &str/string，value: &Decimal/Decimal；回傳 `Result<(), ValidationError>`。JS 成功回傳 undefined；失敗拋出 ValidationError，帶 category="validation" 及 field/tag/parameter 屬性，區別於 DecimalError。保留欄位名和原參數文字，包括花括號與空白。借用規則及數值。需要無異常收集多個錯誤時使用 validate_decimal。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationrule-validate" locale="zh-Hant" />

## 瀏覽器邊界

validation feature 已啟用。單個字串最多 8192 個 UTF-8 位元組；規則清單、錯誤清單和訊息映射最多 64 項，JSON 傳輸最多 65536 位元組。小數規則參數遵循 [十進位資源限制](../resource-policy)。翻譯前保守估計預留位置展開後的總文字，超過 65536 位元組時拒絕；可能拒絕實際剛好容納的結果。自訂類別和欄位是資料，不執行程式碼。驗證失敗與組態錯誤、bridge 資源錯誤分開報告。

## 相關方法

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
