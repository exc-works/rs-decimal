---
locale: zh-Hans
source_revision: 356d7409ac746daa69113ece073c46b80c7b34815cbe6ddfc49fc1c80e686226
api_id: "validation::ValidationError.field"
---
# validation::ValidationError.field

查看失败验证记录。

```rust
field: String
```

## 参数、返回值及常规用法

公开的自有 String/string，保存调用者提供的字段名。不是属性查找路径或 HTML 片段。翻译时按字面插入花括号。修改错误记录不会改变规则或小数。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror-field" locale="zh-Hans" />

## 相关方法

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
