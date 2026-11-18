---
locale: zh-Hans
source_revision: 96429a6b772c7e5f91528e442f9be3179ac231b38327e61764d5f24018444428
api_id: "reexport::validation::default_validation_messages"
---
# reexport::validation::default_validation_messages

获取新的语言模板映射。

```rust
pub use validation::messages::default_validation_messages as default_validation_messages
```

## 参数、返回值及常规用法

locale: &str/string；返回 `BTreeMap<String,String>`/普通对象，包含全部十二种规则模板。未知语言使用英文。每次调用持有独立映射，修改不会影响后续调用或其他 ValidationMessages。重导出是同一函数在公开模块中的导入路径。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-reexport-validation-default-validation-messages" locale="zh-Hans" />

## 浏览器边界

validation feature 已启用。单个字符串最多 8192 个 UTF-8 字节；规则列表、错误列表和消息映射最多 64 项，JSON 传输最多 65536 字节。小数规则参数遵循 [十进制资源限制](../resource-policy)。翻译前保守估计占位符展开后的总文本，超过 65536 字节时拒绝；可能拒绝实际刚好容纳的结果。自定义类别和字段是数据，不执行代码。验证失败与配置错误、bridge 资源错误分开报告。

## 相关方法

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
