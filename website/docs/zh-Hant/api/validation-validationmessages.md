---
locale: zh-Hant
source_revision: 9937fd6977e8a466e16b6a35082706b1183e93c9c873b5eb8e6c66d5ab917b4f
translation_provenance: zh-Hans-terminology-reviewed
api_id: "validation::ValidationMessages"
---
# validation::ValidationMessages

透過自有模板翻譯驗證失敗。

```rust
pub struct ValidationMessages
```

## 參數、回傳值及常規用法

持有用於翻譯 ValidationError 的模板。透過 new(locale) 或 with_messages(map) 建構；修改其他模板映射不影響此控制代碼。語言匹配忽略大小寫，接受連字元、底線和語言區域後綴；zh-Hant/zh-TW 選擇繁體中文。en-US/en-GB 使用英文；未知語言回退到英文。{0} 插入欄位，{1} 插入原參數，只替換一次，插入文字中的花括號不再次展開。回傳純文字，應按文字顯示，不作為 HTML。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationmessages" locale="zh-Hant" />

## 相關方法

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
