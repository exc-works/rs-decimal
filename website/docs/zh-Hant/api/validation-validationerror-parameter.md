---
locale: zh-Hant
source_revision: 9d8774ce76acc603ce438d5ebe120fafe780695ef55b74ea55b1fbe183940981
translation_provenance: zh-Hans-terminology-reviewed
api_id: "validation::ValidationError.parameter"
---
# validation::ValidationError.parameter

查看失敗驗證記錄。

```rust
parameter: String
```

## 參數、回傳值及常規用法

公開的自有 String/string，精確保留原始規則參數，包括空白。作為 {1} 只替換一次，不遞迴解釋模板。修改錯誤記錄不會改變規則或小數。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror-parameter" locale="zh-Hant" />

## 相關方法

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
