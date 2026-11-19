---
locale: zh-Hant
source_revision: 906bb0bd7555c68ed2da4c7be205d647a1ca94b724e2e70f5d1160d968357278
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

## 相關方法

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
