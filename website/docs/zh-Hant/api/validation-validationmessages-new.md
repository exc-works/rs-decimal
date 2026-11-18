---
locale: zh-Hant
source_revision: d38eb31907db7ef2944305de43af506a13542551be2fcba604fea1ed0a54e78f
translation_provenance: zh-Hans-terminology-reviewed
api_id: "validation::ValidationMessages::new"
---
# validation::ValidationMessages::new

透過自有模板翻譯驗證失敗。

```rust
fn new (locale : & str) -> Self
```

## 參數、回傳值及常規用法

locale: &str/string；回傳新的自有訊息模板。語言匹配忽略大小寫，接受連字元、底線和語言區域後綴；zh-Hant/zh-TW 選擇繁體中文。en-US/en-GB 使用英文；未知語言回退到英文。{0} 插入欄位，{1} 插入原參數，只替換一次，插入文字中的花括號不再次展開。回傳純文字，應按文字顯示，不作為 HTML。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationmessages-new" locale="zh-Hant" />

## 瀏覽器邊界

validation feature 已啟用。單個字串最多 8192 個 UTF-8 位元組；規則清單、錯誤清單和訊息映射最多 64 項，JSON 傳輸最多 65536 位元組。小數規則參數遵循 [十進位資源限制](../resource-policy)。翻譯前保守估計預留位置展開後的總文字，超過 65536 位元組時拒絕；可能拒絕實際剛好容納的結果。自訂類別和欄位是資料，不執行程式碼。驗證失敗與組態錯誤、bridge 資源錯誤分開報告。

## 相關方法

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
