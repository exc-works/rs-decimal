---
locale: zh-Hant
source_revision: 8ffc267a7c40a9915ad1544633aa5847696df2f114f2a6275b83514da10320eb
translation_provenance: zh-Hans-terminology-reviewed
api_id: "validation::ValidationRule::validate"
---
# validation::ValidationRule::validate

按規則驗證一個欄位。

```rust
fn validate (& self , field : & str , value : & Decimal) -> Result < () , ValidationError >
```

## 參數、回傳值及常規用法

field: &str/string，value: &Decimal/Decimal；回傳 `Result<(), ValidationError>`。JS 成功回傳 undefined；失敗拋出 ValidationError，帶 category="validation" 及 field/tag/parameter 屬性，區別於 DecimalError。保留欄位名和原參數文字，包括花括號與空白。借用規則及數值。需要無異常收集多個錯誤時使用 validate_decimal。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationrule-validate" locale="zh-Hant" />

## 相關方法

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
