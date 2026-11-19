---
locale: zh-Hans
source_revision: 0da87c25bd0ca11269721d2b8d815d7bac612f73d7110e87ce497372ebbaf01e
api_id: "validation::ValidationError::impl Debug"
---
# validation::ValidationError::impl Debug

使用 Rust Debug 实现。

```rust
impl Debug for validation::ValidationError
```

## 参数、返回值及常规用法

Debug 产生诊断文本，不是稳定的验证序列化格式。这是原生 Rust trait 语法，不是浏览器方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror-impl-debug" locale="zh-Hans" />

## 相关方法

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
