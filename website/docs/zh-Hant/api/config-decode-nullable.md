---
locale: zh-Hant
source_revision: 324990894f6327d9f0e88937d867ea0f32bc3c792ba37210279bca0b73495f73
translation_provenance: zh-Hans-terminology-reviewed
api_id: "config::decode_nullable"
---
# config::decode_nullable

解碼可空組態值。

```rust
fn decode_nullable (value : ConfigValue < '_ >) -> Result < NullDecimal , DecimalError >
```

## 參數、回傳值及常規用法

value: ConfigValue；回傳 `Result<NullDecimal, DecimalError>`/新的可空控制代碼。Null、恰好為空的 Text 和空 Bytes 產生無效值，內部零未初始化。只有空白的 Text 和空 Number 仍會解析失敗。現有 Nullable 複製有效旗標和隱藏內部資料，即使原值無效；借用 Decimal 時結果有效，但不會強制初始化。其他輸入遵循 decode_decimal，成功後變為有效。輸入不被消耗。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-decode-nullable" locale="zh-Hant" />

## 相關方法

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
