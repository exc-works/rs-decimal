---
locale: zh-Hans
source_revision: 572356860c801a3a257544cc180e1dca7bec96afecfd793ab3dbbcce432242dd
api_id: "validation::ValidationMessages::impl Clone"
---
# validation::ValidationMessages::impl Clone

使用 Rust Clone 实现。

```rust
impl Clone for validation::ValidationMessages
```

## 参数、返回值及常规用法

Clone 复制完整规则、模板映射或失败字段，副本独立持有。这是原生 Rust trait 语法，不是浏览器方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationmessages-impl-clone" locale="zh-Hans" />

## 相关方法

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
