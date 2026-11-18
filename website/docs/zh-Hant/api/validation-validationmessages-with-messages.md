---
locale: zh-Hant
source_revision: 4973633c1cdc3ddfb696d250ca871028b9835fa4187520f393245e4690f6a0c3
translation_provenance: zh-Hans-terminology-reviewed
api_id: "validation::ValidationMessages::with_messages"
---
# validation::ValidationMessages::with_messages

覆蓋英文驗證模板。

```rust
fn with_messages (messages : BTreeMap < String , String >) -> Result < Self , DecimalError >
```

## 參數、回傳值及常規用法

messages: `BTreeMap<String,String>`/字串值物件；回傳 `Result<ValidationMessages, DecimalError>`/訊息控制代碼。只覆蓋提供的鍵，其他規則保留英文預設值。鍵限於十二種 decimal_* 規則。空模板、未知鍵、不匹配花括號及 {0}/{1} 以外的預留位置回傳 InvalidArgument。允許重複有效預留位置。不改變全域或已建構的模板。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationmessages-with-messages" locale="zh-Hant" />

## 瀏覽器邊界

validation feature 已啟用。單個字串最多 8192 個 UTF-8 位元組；規則清單、錯誤清單和訊息映射最多 64 項，JSON 傳輸最多 65536 位元組。小數規則參數遵循 [十進位資源限制](../resource-policy)。翻譯前保守估計預留位置展開後的總文字，超過 65536 位元組時拒絕；可能拒絕實際剛好容納的結果。自訂類別和欄位是資料，不執行程式碼。驗證失敗與組態錯誤、bridge 資源錯誤分開報告。

## 相關方法

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
