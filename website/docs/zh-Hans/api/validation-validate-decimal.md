---
locale: zh-Hans
source_revision: 65b82f1df9828e1e76dbeae405f27c44161cfdcb090640f8812099c14a0bd130
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

## 浏览器边界

validation feature 已启用。单个字符串最多 8192 个 UTF-8 字节；规则列表、错误列表和消息映射最多 64 项，JSON 传输最多 65536 字节。小数规则参数遵循 [十进制资源限制](../resource-policy)。翻译前保守估计占位符展开后的总文本，超过 65536 字节时拒绝；可能拒绝实际刚好容纳的结果。自定义类别和字段是数据，不执行代码。验证失败与配置错误、bridge 资源错误分开报告。

## 相关方法

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
