---
locale: zh-Hant
source_revision: 9e6af4fbdd99e4ba366c644aaf4e403edbed7726d41f29fd990845c2a190132b
translation_provenance: zh-Hans-terminology-reviewed
api_id: "config::ConfigValue::Unsigned"
---
# config::ConfigValue::Unsigned

選擇 Unsigned 組態輸入。

```rust
Unsigned (u128)
```

## 參數、回傳值及常規用法

傳遞精確無符號 u128。瀏覽器要求 [0, 2^128-1] 範圍的 bigint，不經過浮點數。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-unsigned" locale="zh-Hant" />

## 相關方法

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
