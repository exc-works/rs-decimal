---
locale: zh-Hans
source_revision: 7f1c2832917badced6e78a9c287520234452b297109e8406a028c3703581c859
api_id: "validation::ValidationError::impl PartialEq"
---
# validation::ValidationError::impl PartialEq

使用 Rust PartialEq 实现。

```rust
impl PartialEq for validation::ValidationError
```

## 参数、返回值及常规用法

ValidationError 相等同时比较 field、tag 和原始 parameter，不是只比较翻译后的消息。这是原生 Rust trait 语法，不是浏览器方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror-impl-partialeq" locale="zh-Hans" />

## 相关方法

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
