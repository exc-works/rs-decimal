---
locale: zh-Hans
source_revision: 668ba6d01bb4e8f8fc531b677bb3554fa942e3fa2858410dfc728efffb55ae7f
api_id: "validation"
---
# validation

按顺序收集规则失败并翻译。

```rust
pub mod validation
```

## 参数、返回值及常规用法

Rust 需要 validation feature，浏览器构建已启用。此模块提供精确规则验证和自有消息模板，无需框架或全局注册表。浏览器通过 validation 命名空间访问。字段名由调用者提供，翻译时按字面插入。缺失输入必须失败时应明确添加 required。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation" locale="zh-Hans" />

## 相关方法

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
