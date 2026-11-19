---
locale: zh-Hans
source_revision: 2b24eac398b765662bae8967979a2547332ee566c8d7e9c0bb785ba0d1f9e478
api_id: "validation::ValidationMessages::new"
---
# validation::ValidationMessages::new

通过自有模板翻译验证失败。

```rust
fn new (locale : & str) -> Self
```

## 参数、返回值及常规用法

locale: &str/string；返回新的自有消息模板。语言匹配忽略大小写，接受连字符、下划线和语言区域后缀；zh-Hant/zh-TW 选择繁体中文。en-US/en-GB 使用英文；未知语言回退到英文。{0} 插入字段，{1} 插入原参数，只替换一次，插入文本中的花括号不再次展开。返回纯文本，应按文本显示，不作为 HTML。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationmessages-new" locale="zh-Hans" />

## 相关方法

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
