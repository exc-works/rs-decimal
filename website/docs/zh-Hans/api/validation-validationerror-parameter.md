---
locale: zh-Hans
source_revision: 9d8774ce76acc603ce438d5ebe120fafe780695ef55b74ea55b1fbe183940981
api_id: "validation::ValidationError.parameter"
---
# validation::ValidationError.parameter

查看失败验证记录。

```rust
parameter: String
```

## 参数、返回值及常规用法

公开的自有 String/string，精确保留原始规则参数，包括空白。作为 {1} 只替换一次，不递归解释模板。修改错误记录不会改变规则或小数。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror-parameter" locale="zh-Hans" />

## 相关方法

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
