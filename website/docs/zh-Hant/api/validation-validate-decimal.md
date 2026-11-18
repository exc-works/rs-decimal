---
locale: zh-Hant
source_revision: 65b82f1df9828e1e76dbeae405f27c44161cfdcb090640f8812099c14a0bd130
translation_provenance: zh-Hans-terminology-reviewed
api_id: "validation::validate_decimal"
---
# validation::validate_decimal

按順序收集規則失敗並翻譯。

```rust
fn validate_decimal (field : & str , value : & Decimal , rules : & [ValidationRule] ,) -> Vec < ValidationError >
```

## 參數、回傳值及常規用法

Rust 需要 validation feature，瀏覽器建置已啟用。field: &str/string，value: &Decimal/Decimal，rules: &[ValidationRule]/陣列；按呼叫者順序回傳所有失敗的 ValidationError。重複規則可產生重複錯誤；不依賴反射或註冊表，不提前退出。空規則回傳空陣列。借用全部輸入。欄位名由呼叫者提供，翻譯時按字面插入。缺失輸入必須失敗時應明確添加 required。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validate-decimal" locale="zh-Hant" />

## 瀏覽器邊界

validation feature 已啟用。單個字串最多 8192 個 UTF-8 位元組；規則清單、錯誤清單和訊息映射最多 64 項，JSON 傳輸最多 65536 位元組。小數規則參數遵循 [十進位資源限制](../resource-policy)。翻譯前保守估計預留位置展開後的總文字，超過 65536 位元組時拒絕；可能拒絕實際剛好容納的結果。自訂類別和欄位是資料，不執行程式碼。驗證失敗與組態錯誤、bridge 資源錯誤分開報告。

## 相關方法

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
