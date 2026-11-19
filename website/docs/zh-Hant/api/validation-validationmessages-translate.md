---
locale: zh-Hant
source_revision: d21e2ccb9059c66b32b65cffe1a9b447ab9a7fc0664cf5fa020b0d334fe294fd
translation_provenance: zh-Hans-terminology-reviewed
api_id: "validation::ValidationMessages::translate"
---
# validation::ValidationMessages::translate

透過自有模板翻譯驗證失敗。

```rust
fn translate (& self , errors : & [ValidationError]) -> Vec < String >
```

## 參數、回傳值及常規用法

errors: &[ValidationError]/陣列；按同一順序回傳字串陣列。語言匹配忽略大小寫，接受連字元、底線和語言區域後綴；zh-Hant/zh-TW 選擇繁體中文。en-US/en-GB 使用英文；未知語言回退到英文。{0} 插入欄位，{1} 插入原參數，只替換一次，插入文字中的花括號不再次展開。回傳純文字，應按文字顯示，不作為 HTML。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationmessages-translate" locale="zh-Hant" />

## 相關方法

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
