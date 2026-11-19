---
locale: zh-Hant
source_revision: 5a5d502f0868797711c2e17ee061bb453dff465719822a493b3a33d226cf87b4
translation_provenance: zh-Hans-terminology-reviewed
api_id: "config::ConfigValue::Nullable"
---
# config::ConfigValue::Nullable

選擇 Nullable 組態輸入。

```rust
Nullable (& 'a NullDecimal)
```

## 參數、回傳值及常規用法

借用現有 nullable。decode_nullable 複製有效旗標和隱藏資料；decode_decimal 只接受有效值並複製內部 Decimal。解碼前的後續修改會被觀察到。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-nullable" locale="zh-Hant" />

## 相關方法

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
