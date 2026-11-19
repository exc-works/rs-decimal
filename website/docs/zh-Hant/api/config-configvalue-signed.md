---
locale: zh-Hant
source_revision: 07611a32fa9c50a924b808d879544951b6661718e9f82991f25c28bfa7a925e8
translation_provenance: zh-Hans-terminology-reviewed
api_id: "config::ConfigValue::Signed"
---
# config::ConfigValue::Signed

選擇 Signed 組態輸入。

```rust
Signed (i128)
```

## 參數、回傳值及常規用法

傳遞精確有符號 i128。瀏覽器要求 [-2^127, 2^127-1] 範圍的 bigint；轉換前拒絕 JS number 和越界值。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-signed" locale="zh-Hant" />

## 相關方法

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
