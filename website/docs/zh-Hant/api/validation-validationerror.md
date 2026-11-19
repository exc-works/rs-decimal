---
locale: zh-Hant
source_revision: 521743c60e4285630a1e202d92aebd0d0b9f4764a73c56a223f9ec0c5662a6d6
translation_provenance: zh-Hans-terminology-reviewed
api_id: "validation::ValidationError"
---
# validation::ValidationError

查看失敗驗證記錄。

```rust
pub struct ValidationError
```

## 參數、回傳值及常規用法

失敗記錄包含公開 field、tag、parameter 屬性。ValidationRule.validate 失敗時回傳，validate_decimal 回傳其清單。JS 提供可編輯欄位記錄，用於翻譯。它們不是 DecimalError 類別或異常訊息。修改錯誤記錄不會改變規則或小數。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror" locale="zh-Hant" />

## 相關方法

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
