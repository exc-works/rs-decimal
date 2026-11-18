---
locale: zh-Hant
source_revision: 6fae0dc88a563ce1dfaf67cbb1ece08366df9aad8969e9320dabf2cf06fa5933
translation_provenance: zh-Hans-terminology-reviewed
api_id: "validation::ValidationError.tag"
---
# validation::ValidationError.tag

查看失敗驗證記錄。

```rust
tag: & 'static str
```

## 參數、回傳值及常規用法

Rust 公開靜態字串，識別碼失敗規則。瀏覽器使用自有字串屬性；未知標籤翻譯為核心預設模板 "{0} is invalid"。修改錯誤記錄不會改變規則或小數。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror-tag" locale="zh-Hant" />

## 瀏覽器邊界

validation feature 已啟用。單個字串最多 8192 個 UTF-8 位元組；規則清單、錯誤清單和訊息映射最多 64 項，JSON 傳輸最多 65536 位元組。小數規則參數遵循 [十進位資源限制](../resource-policy)。翻譯前保守估計預留位置展開後的總文字，超過 65536 位元組時拒絕；可能拒絕實際剛好容納的結果。自訂類別和欄位是資料，不執行程式碼。驗證失敗與組態錯誤、bridge 資源錯誤分開報告。

## 相關方法

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
