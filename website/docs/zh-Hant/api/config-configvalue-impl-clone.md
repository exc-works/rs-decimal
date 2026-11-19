---
locale: zh-Hant
source_revision: db26ba148bd1b6fe560088f6be1bce313faff866656299f7c24eeb47568b46bf
translation_provenance: zh-Hans-terminology-reviewed
api_id: "config::ConfigValue::impl Clone"
---
# config::ConfigValue::impl Clone

使用原生 Clone 實現。

```rust
impl Clone for config::ConfigValue
```

## 參數、回傳值及常規用法

ConfigValue 是小型類型列舉。Clone/Copy 保留借用參照，不深拷貝參照目標。Debug 是診斷文字，不是穩定的傳輸格式。原生 Rust trait 語法單獨測試，不是瀏覽器方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-impl-clone" locale="zh-Hant" />

## 相關方法

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
