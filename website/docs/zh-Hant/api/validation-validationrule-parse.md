---
locale: zh-Hant
source_revision: e175d4d7de2a038dff837a099025c66d8337b4480eacf9bc8d2de1f1d5950025
translation_provenance: zh-Hans-terminology-reviewed
api_id: "validation::ValidationRule::parse"
---
# validation::ValidationRule::parse

執行精確的小數規則。

```rust
fn parse (tag : & str , parameter : & str) -> Result < Self , DecimalError >
```

## 參數、回傳值及常規用法

tag: &str/string，parameter: &str/string；回傳 `Result<ValidationRule, DecimalError>`/規則控制代碼。使用 decimal_gt 等完整標籤。比較參數為小數文字，between 使用 lower~upper，max_precision 使用非負 isize。未知標籤、錯誤參數和反向區間回傳 InvalidArgument。在 wasm32 上 isize 為有符號 32 位。不需要參數的規則保留但不解釋參數文字。規則精確比較 Decimal，不經過浮點轉換。decimal_required 檢查初始化，因此已初始化零透過。eq/ne/gt/gte/lt/lte 按數值比較，包括未初始化零。between 使用包含端點的 lower~upper，且額外要求初始化。positive/negative/nonzero 要求初始化。max_precision 檢查儲存的 scale，包括保留的末尾零；它不隱含 required。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationrule-parse" locale="zh-Hant" />

## 瀏覽器邊界

validation feature 已啟用。單個字串最多 8192 個 UTF-8 位元組；規則清單、錯誤清單和訊息映射最多 64 項，JSON 傳輸最多 65536 位元組。小數規則參數遵循 [十進位資源限制](../resource-policy)。翻譯前保守估計預留位置展開後的總文字，超過 65536 位元組時拒絕；可能拒絕實際剛好容納的結果。自訂類別和欄位是資料，不執行程式碼。驗證失敗與組態錯誤、bridge 資源錯誤分開報告。

## 相關方法

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
