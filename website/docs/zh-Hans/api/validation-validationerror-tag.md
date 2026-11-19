---
locale: zh-Hans
source_revision: 0b44060089288355d3eba23edfcc95ed4174aef13953ff7dc6b74df01f566b11
api_id: "validation::ValidationError.tag"
---
# validation::ValidationError.tag

查看失败验证记录。

```rust
tag: & 'static str
```

## 参数、返回值及常规用法

Rust 公开静态字符串，标识失败规则。浏览器使用自有字符串属性；未知标签翻译为核心默认模板 "{0} is invalid"。修改错误记录不会改变规则或小数。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror-tag" locale="zh-Hans" />

## 相关方法

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
