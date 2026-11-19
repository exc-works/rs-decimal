---
locale: zh-Hant
source_revision: e064dade2acfdfd8dc8224a07d00162d5bdf94c66ca955bda753d76d5540ec7b
translation_provenance: zh-Hans-terminology-reviewed
api_id: "config::ConfigValue::Float32"
---
# config::ConfigValue::Float32

選擇 Float32 組態輸入。

```rust
Float32 (f32)
```

## 參數、回傳值及常規用法

傳遞 binary32。Rust 先將瀏覽器 number 收窄為 f32，再使用核心轉換。非有限輸入或收窄溢出回傳 Unmarshal。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-float32" locale="zh-Hant" />

## 相關方法

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
