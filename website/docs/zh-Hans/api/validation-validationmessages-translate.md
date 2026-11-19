---
locale: zh-Hans
source_revision: d21e2ccb9059c66b32b65cffe1a9b447ab9a7fc0664cf5fa020b0d334fe294fd
api_id: "validation::ValidationMessages::translate"
---
# validation::ValidationMessages::translate

通过自有模板翻译验证失败。

```rust
fn translate (& self , errors : & [ValidationError]) -> Vec < String >
```

## 参数、返回值及常规用法

errors: &[ValidationError]/数组；按同一顺序返回字符串数组。语言匹配忽略大小写，接受连字符、下划线和语言区域后缀；zh-Hant/zh-TW 选择繁体中文。en-US/en-GB 使用英文；未知语言回退到英文。{0} 插入字段，{1} 插入原参数，只替换一次，插入文本中的花括号不再次展开。返回纯文本，应按文本显示，不作为 HTML。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationmessages-translate" locale="zh-Hans" />

## 相关方法

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
