---
locale: zh-Hans
source_revision: 53a002a881140d83f1a447d3f2f73f066b75a4b8a71f55b044742e2243254251
api_id: "validation::validate_decimal"
---
# validation::validate_decimal

按顺序收集规则失败并翻译。

```rust
fn validate_decimal (field : & str , value : & Decimal , rules : & [ValidationRule] ,) -> Vec < ValidationError >
```

## 参数、返回值及常规用法

Rust 需要 validation feature，浏览器构建已启用。field: &str/string，value: &Decimal/Decimal，rules: &[ValidationRule]/数组；按调用者顺序返回所有失败的 ValidationError。重复规则可产生重复错误；不依赖反射或注册表，不提前退出。空规则返回空数组。借用全部输入。字段名由调用者提供，翻译时按字面插入。缺失输入必须失败时应明确添加 required。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validate-decimal" locale="zh-Hans" />

## 相关方法

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
