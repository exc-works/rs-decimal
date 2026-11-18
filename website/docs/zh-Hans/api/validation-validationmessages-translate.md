---
locale: zh-Hans
source_revision: cba611f08bedeba221d0757fa2de9adc9759708b4a544bb24d89caed0ec075de
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

## 浏览器边界

validation feature 已启用。单个字符串最多 8192 个 UTF-8 字节；规则列表、错误列表和消息映射最多 64 项，JSON 传输最多 65536 字节。小数规则参数遵循 [十进制资源限制](../resource-policy)。翻译前保守估计占位符展开后的总文本，超过 65536 字节时拒绝；可能拒绝实际刚好容纳的结果。自定义类别和字段是数据，不执行代码。验证失败与配置错误、bridge 资源错误分开报告。

## 相关方法

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
