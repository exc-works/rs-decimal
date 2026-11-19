---
locale: zh-Hans
source_revision: 262ffddf12b7585e3d9a91337def5d4995343306864bd291c3fadc2686ec5701
api_id: "validation::ValidationError::impl Clone"
---
# validation::ValidationError::impl Clone

使用 Rust Clone 实现。

```rust
impl Clone for validation::ValidationError
```

## 参数、返回值及常规用法

Clone 复制完整规则、模板映射或失败字段，副本独立持有。这是原生 Rust trait 语法，不是浏览器方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror-impl-clone" locale="zh-Hans" />

## 相关方法

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
