---
locale: zh-Hans
source_revision: cfe29d83921c9d9fef3ddeed3842336ccc12799d1b0ed231fac999e4d87de7f7
api_id: "validation::ValidationError::impl Eq"
---
# validation::ValidationError::impl Eq

使用 Rust Eq 实现。

```rust
impl Eq for validation::ValidationError
```

## 参数、返回值及常规用法

ValidationError 相等同时比较 field、tag 和原始 parameter，不是只比较翻译后的消息。这是原生 Rust trait 语法，不是浏览器方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror-impl-eq" locale="zh-Hans" />

## 相关方法

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
