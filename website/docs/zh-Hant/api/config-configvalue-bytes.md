---
locale: zh-Hant
source_revision: bfcaf340b0c1c1671610ce3a1025921897c59afd9bdf91da4be0b86f5f4d7446
translation_provenance: zh-Hans-terminology-reviewed
api_id: "config::ConfigValue::Bytes"
---
# config::ConfigValue::Bytes

選擇 Bytes 組態輸入。

```rust
Bytes (& 'a [u8])
```

## 參數、回傳值及常規用法

借用小數文字位元組，不是二進位小數負載。瀏覽器觀察解碼前的 Uint8Array 修改，解碼不修改陣列。無效 UTF-8 導致小數解析失敗。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-bytes" locale="zh-Hant" />

## 相關方法

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
