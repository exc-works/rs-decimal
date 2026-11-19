---
locale: zh-Hant
source_revision: 816a5e8449654efb901339ad5516b3fdb94be2af25e3c709a61823154b83be91
translation_provenance: zh-Hans-terminology-reviewed
api_id: "validation::ValidationRule::matches"
---
# validation::ValidationRule::matches

執行精確的小數規則。

```rust
fn matches (& self , value : & Decimal) -> bool
```

## 參數、回傳值及常規用法

value: &Decimal/Decimal；回傳 bool/boolean，並保留規則與數值。規則精確比較 Decimal，不經過浮點轉換。decimal_required 檢查初始化，因此已初始化零透過。eq/ne/gt/gte/lt/lte 按數值比較，包括未初始化零。between 使用包含端點的 lower~upper，且額外要求初始化。positive/negative/nonzero 要求初始化。max_precision 檢查儲存的 scale，包括保留的末尾零；它不隱含 required。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationrule-matches" locale="zh-Hant" />

## 相關方法

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
