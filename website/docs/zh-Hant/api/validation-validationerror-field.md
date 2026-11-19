---
locale: zh-Hant
source_revision: 356d7409ac746daa69113ece073c46b80c7b34815cbe6ddfc49fc1c80e686226
translation_provenance: zh-Hans-terminology-reviewed
api_id: "validation::ValidationError.field"
---
# validation::ValidationError.field

查看失敗驗證記錄。

```rust
field: String
```

## 參數、回傳值及常規用法

公開的自有 String/string，儲存呼叫者提供的欄位名。不是屬性查找路徑或 HTML 片段。翻譯時按字面插入花括號。修改錯誤記錄不會改變規則或小數。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror-field" locale="zh-Hant" />

## 相關方法

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
