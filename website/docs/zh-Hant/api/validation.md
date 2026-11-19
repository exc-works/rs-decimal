---
locale: zh-Hant
source_revision: 668ba6d01bb4e8f8fc531b677bb3554fa942e3fa2858410dfc728efffb55ae7f
translation_provenance: zh-Hans-terminology-reviewed
api_id: "validation"
---
# validation

按順序收集規則失敗並翻譯。

```rust
pub mod validation
```

## 參數、回傳值及常規用法

Rust 需要 validation feature，瀏覽器建置已啟用。此模組提供精確規則驗證和自有訊息模板，無需框架或全域註冊表。瀏覽器透過 validation 命名空間存取。欄位名由呼叫者提供，翻譯時按字面插入。缺失輸入必須失敗時應明確添加 required。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation" locale="zh-Hant" />

## 相關方法

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
