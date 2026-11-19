---
locale: zh-Hant
source_revision: 0b44060089288355d3eba23edfcc95ed4174aef13953ff7dc6b74df01f566b11
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

## 相關方法

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
