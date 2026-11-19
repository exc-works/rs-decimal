---
locale: zh-Hant
source_revision: 4a04c1296f4a9e4d0c1eac8d1707242f3d7b87d7302eac99dc6a590f2c03208e
translation_provenance: zh-Hans-terminology-reviewed
api_id: "validation::ValidationRule"
---
# validation::ValidationRule

執行精確的小數規則。

```rust
pub struct ValidationRule
```

## 參數、回傳值及常規用法

透過 parse(tag, parameter) 建立規則，再重複使用於 matches 或 validate。建構時檢查參數，瀏覽器控制代碼屬於本次執行，可以顯式釋放。規則精確比較 Decimal，不經過浮點轉換。decimal_required 檢查初始化，因此已初始化零透過。eq/ne/gt/gte/lt/lte 按數值比較，包括未初始化零。between 使用包含端點的 lower~upper，且額外要求初始化。positive/negative/nonzero 要求初始化。max_precision 檢查儲存的 scale，包括保留的末尾零；它不隱含 required。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationrule" locale="zh-Hant" />

## 相關方法

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
