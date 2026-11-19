---
locale: zh-Hant
source_revision: 39668a70027991d27730b01fd29253411ac796d1b8a82a794c00f60c687cf5f8
translation_provenance: zh-Hans-terminology-reviewed
api_id: "config::ConfigValue::impl Copy"
---
# config::ConfigValue::impl Copy

使用原生 Copy 實現。

```rust
impl Copy for config::ConfigValue
```

## 參數、回傳值及常規用法

ConfigValue 是小型類型列舉。Clone/Copy 保留借用參照，不深拷貝參照目標。Debug 是診斷文字，不是穩定的傳輸格式。原生 Rust trait 語法單獨測試，不是瀏覽器方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-impl-copy" locale="zh-Hant" />

## 相關方法

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
