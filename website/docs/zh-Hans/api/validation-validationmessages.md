---
locale: zh-Hans
source_revision: 9937fd6977e8a466e16b6a35082706b1183e93c9c873b5eb8e6c66d5ab917b4f
api_id: "validation::ValidationMessages"
---
# validation::ValidationMessages

通过自有模板翻译验证失败。

```rust
pub struct ValidationMessages
```

## 参数、返回值及常规用法

持有用于翻译 ValidationError 的模板。通过 new(locale) 或 with_messages(map) 构造；修改其他模板映射不影响此句柄。语言匹配忽略大小写，接受连字符、下划线和语言区域后缀；zh-Hant/zh-TW 选择繁体中文。en-US/en-GB 使用英文；未知语言回退到英文。{0} 插入字段，{1} 插入原参数，只替换一次，插入文本中的花括号不再次展开。返回纯文本，应按文本显示，不作为 HTML。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationmessages" locale="zh-Hans" />

## 相关方法

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
