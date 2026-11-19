---
locale: zh-Hant
source_revision: dafd4579c57c7280a7d3e288044769361224c3db24eb3a6174eb3833dfe2bd19
translation_provenance: zh-Hans-terminology-reviewed
api_id: "config::ConfigValue::Unsupported"
---
# config::ConfigValue::Unsupported

選擇 Unsupported 組態輸入。

```rust
Unsupported
```

## 參數、回傳值及常規用法

明確表示不支援的組態類型，不做強制轉換。兩個 decode 函數都回傳 Unmarshal。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-unsupported" locale="zh-Hant" />

## 相關方法

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
