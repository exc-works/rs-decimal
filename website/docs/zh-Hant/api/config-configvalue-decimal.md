---
locale: zh-Hant
source_revision: 8d101e011ba2e4f441ebfc6371c73cb22589c6224599cc87e367af949b20dc8f
translation_provenance: zh-Hans-terminology-reviewed
api_id: "config::ConfigValue::Decimal"
---
# config::ConfigValue::Decimal

選擇 Decimal 組態輸入。

```rust
Decimal (& 'a Decimal)
```

## 參數、回傳值及常規用法

借用現有小數，在解碼時複製係數、scale 和初始化狀態。來源值仍可使用。瀏覽器來源值在解碼前釋放或移動後，該借用變體不可用。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-decimal" locale="zh-Hant" />

## 相關方法

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
