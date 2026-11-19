---
locale: zh-Hant
source_revision: 846797abc20f334aac55a6cc16f18858d3e8f7870cd49af2d7f236e4e0d093b3
translation_provenance: zh-Hans-terminology-reviewed
api_id: "config::ConfigValue::Null"
---
# config::ConfigValue::Null

選擇 Null 組態輸入。

```rust
Null
```

## 參數、回傳值及常規用法

缺失的組態輸入。decode_nullable 回傳無效值，decode_decimal 回傳 Unmarshal。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-null" locale="zh-Hant" />

## 相關方法

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
