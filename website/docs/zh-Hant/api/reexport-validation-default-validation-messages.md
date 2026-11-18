---
locale: zh-Hant
source_revision: 96429a6b772c7e5f91528e442f9be3179ac231b38327e61764d5f24018444428
translation_provenance: zh-Hans-terminology-reviewed
api_id: "reexport::validation::default_validation_messages"
---
# reexport::validation::default_validation_messages

取得新的語言模板映射。

```rust
pub use validation::messages::default_validation_messages as default_validation_messages
```

## 參數、回傳值及常規用法

locale: &str/string；回傳 `BTreeMap<String,String>`/普通物件，包含全部十二種規則模板。未知語言使用英文。每次呼叫持有獨立映射，修改不會影響後續呼叫或其他 ValidationMessages。重匯出是同一函數在公開模組中的匯入路徑。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-reexport-validation-default-validation-messages" locale="zh-Hant" />

## 瀏覽器邊界

validation feature 已啟用。單個字串最多 8192 個 UTF-8 位元組；規則清單、錯誤清單和訊息映射最多 64 項，JSON 傳輸最多 65536 位元組。小數規則參數遵循 [十進位資源限制](../resource-policy)。翻譯前保守估計預留位置展開後的總文字，超過 65536 位元組時拒絕；可能拒絕實際剛好容納的結果。自訂類別和欄位是資料，不執行程式碼。驗證失敗與組態錯誤、bridge 資源錯誤分開報告。

## 相關方法

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
