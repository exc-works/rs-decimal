---
locale: zh-Hant
source_revision: c829b94ceed61e0a65772da94750a53ecb8269151bdb6f1efd6a7c407ad09f12
translation_provenance: zh-Hans-terminology-reviewed
api_id: "reexport::validation::default_validation_messages"
---
# reexport::validation::default_validation_messages

取得新的語言模板映射。

```rust
pub use validation::messages::default_validation_messages as default_validation_messages
```

## 參數、回傳值及常規用法

locale: &str/string；回傳 `BTreeMap<String,String>`/普通物件，包含全部十二種規則模板。未知語言使用英文。每次呼叫持有獨立映射，修改不會影響後續呼叫或其他 ValidationMessages。重匯出是同一函數在公開模組中的匯入路徑。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-reexport-validation-default-validation-messages" locale="zh-Hant" />

## 相關方法

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
