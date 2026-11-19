---
locale: zh-Hant
source_revision: d9b1450ea64f3f424b8c3303a77d21fe7ac3c0fd6d9a478841f3ae7df9627834
translation_provenance: zh-Hans-terminology-reviewed
api_id: "config::ConfigValue::Number"
---
# config::ConfigValue::Number

選擇 Number 組態輸入。

```rust
Number (& 'a str)
```

## 參數、回傳值及常規用法

傳遞數字來源文字，不經過浮點轉換。核心解析小數語法，此變體不是完整 JSON 語法驗證器。空 Number 對 nullable 目標也無效。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-number" locale="zh-Hant" />

## 相關方法

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
