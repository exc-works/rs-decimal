---
locale: zh-Hans
source_revision: b55dba0922642cda4e225685c47d97a54da01243942c47cb892b8c9136202f1a
api_id: "validation::ValidationRule::impl Clone"
---
# validation::ValidationRule::impl Clone

使用 Rust Clone 实现。

```rust
impl Clone for validation::ValidationRule
```

## 参数、返回值及常规用法

Clone 复制完整规则、模板映射或失败字段，副本独立持有。这是原生 Rust trait 语法，不是浏览器方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationrule-impl-clone" locale="zh-Hans" />

## 相关方法

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
