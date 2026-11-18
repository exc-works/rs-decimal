---
locale: zh-Hant
source_revision: 8a30b5196c371952fcc2508159879f3619eaf7085b839003d31c6721b7f9cf86
translation_provenance: zh-Hans-terminology-reviewed
api_id: "validation"
---
# validation

按順序收集規則失敗並翻譯。

```rust
pub mod validation
```

## 參數、回傳值及常規用法

Rust 需要 validation feature，瀏覽器建置已啟用。此模組提供精確規則驗證和自有訊息模板，無需框架或全域註冊表。瀏覽器透過 validation 命名空間存取。欄位名由呼叫者提供，翻譯時按字面插入。缺失輸入必須失敗時應明確添加 required。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation" locale="zh-Hant" />

## 瀏覽器邊界

validation feature 已啟用。單個字串最多 8192 個 UTF-8 位元組；規則清單、錯誤清單和訊息映射最多 64 項，JSON 傳輸最多 65536 位元組。小數規則參數遵循 [十進位資源限制](../resource-policy)。翻譯前保守估計預留位置展開後的總文字，超過 65536 位元組時拒絕；可能拒絕實際剛好容納的結果。自訂類別和欄位是資料，不執行程式碼。驗證失敗與組態錯誤、bridge 資源錯誤分開報告。

## 相關方法

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
