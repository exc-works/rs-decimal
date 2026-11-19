---
locale: zh-Hant
source_revision: 89abd084e8bb8541133194e2046ba4621ebe68fb1c342334ebc3bf5d1ca7dcec
translation_provenance: zh-Hans-terminology-reviewed
api_id: "config::ConfigValue::Text"
---
# config::ConfigValue::Text

選擇 Text 組態輸入。

```rust
Text (& 'a str)
```

## 參數、回傳值及常規用法

借用精確小數文字。首尾空白由小數核心解析，不移除 SQL 風格外圍引號。僅恰好為空的文字在 nullable 解碼時表示缺失。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-text" locale="zh-Hant" />

## 相關方法

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
