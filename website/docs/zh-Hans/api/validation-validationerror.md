---
locale: zh-Hans
source_revision: 521743c60e4285630a1e202d92aebd0d0b9f4764a73c56a223f9ec0c5662a6d6
api_id: "validation::ValidationError"
---
# validation::ValidationError

查看失败验证记录。

```rust
pub struct ValidationError
```

## 参数、返回值及常规用法

失败记录包含公开 field、tag、parameter 属性。ValidationRule.validate 失败时返回，validate_decimal 返回其列表。JS 提供可编辑字段记录，用于翻译。它们不是 DecimalError 类别或异常消息。修改错误记录不会改变规则或小数。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror" locale="zh-Hans" />

## 相关方法

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
