---
locale: zh-Hant
source_revision: 032d932903a81f0f03e2bde5c0ffb35c8a1064335f6ef0a3149d91077fa81075
translation_provenance: zh-Hans-terminology-reviewed
api_id: "config::ConfigValue::Float64"
---
# config::ConfigValue::Float64

選擇 Float64 組態輸入。

```rust
Float64 (f64)
```

## 參數、回傳值及常規用法

透過核心最短十進位轉換傳遞 binary64。非有限數回傳 Unmarshal，已因浮點捨入丟失的數字無法恢復。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-float64" locale="zh-Hant" />

## 相關方法

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
