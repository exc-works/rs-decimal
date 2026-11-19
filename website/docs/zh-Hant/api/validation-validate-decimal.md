---
locale: zh-Hant
source_revision: 53a002a881140d83f1a447d3f2f73f066b75a4b8a71f55b044742e2243254251
translation_provenance: zh-Hans-terminology-reviewed
api_id: "validation::validate_decimal"
---
# validation::validate_decimal

按順序收集規則失敗並翻譯。

```rust
fn validate_decimal (field : & str , value : & Decimal , rules : & [ValidationRule] ,) -> Vec < ValidationError >
```

## 參數、回傳值及常規用法

Rust 需要 validation feature，瀏覽器建置已啟用。field: &str/string，value: &Decimal/Decimal，rules: &[ValidationRule]/陣列；按呼叫者順序回傳所有失敗的 ValidationError。重複規則可產生重複錯誤；不依賴反射或註冊表，不提前退出。空規則回傳空陣列。借用全部輸入。欄位名由呼叫者提供，翻譯時按字面插入。缺失輸入必須失敗時應明確添加 required。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validate-decimal" locale="zh-Hant" />

## 相關方法

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
