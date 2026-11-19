---
locale: zh-Hans
source_revision: 8ffc267a7c40a9915ad1544633aa5847696df2f114f2a6275b83514da10320eb
api_id: "validation::ValidationRule::validate"
---
# validation::ValidationRule::validate

按规则验证一个字段。

```rust
fn validate (& self , field : & str , value : & Decimal) -> Result < () , ValidationError >
```

## 参数、返回值及常规用法

field: &str/string，value: &Decimal/Decimal；返回 `Result<(), ValidationError>`。JS 成功返回 undefined；失败抛出 ValidationError，带 category="validation" 及 field/tag/parameter 属性，区别于 DecimalError。保留字段名和原参数文本，包括花括号与空白。借用规则及数值。需要无异常收集多个错误时使用 validate_decimal。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationrule-validate" locale="zh-Hans" />

## 相关方法

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
