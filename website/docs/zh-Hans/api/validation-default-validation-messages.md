---
locale: zh-Hans
source_revision: db21c0833b97369db1efcc9a73b34501c5dcac820064e9658bbb1cb161d8dec2
api_id: "validation::default_validation_messages"
---
# validation::default_validation_messages

获取新的语言模板映射。

```rust
fn default_validation_messages (locale : & str) -> BTreeMap < String , String >
```

## 参数、返回值及常规用法

locale: &str/string；返回 `BTreeMap<String,String>`/普通对象，包含全部十二种规则模板。未知语言使用英文。每次调用持有独立映射，修改不会影响后续调用或其他 ValidationMessages。重导出是同一函数在公开模块中的导入路径。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-default-validation-messages" locale="zh-Hans" />

## 相关方法

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
