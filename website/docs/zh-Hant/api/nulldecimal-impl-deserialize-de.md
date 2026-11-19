---
locale: zh-Hant
source_revision: 22a6d48f36f7716bfae657ccf09549a7c182a92cb0e3afef6f24a947535edb2b
translation_provenance: zh-Hans-terminology-reviewed
api_id: "NullDecimal::impl Deserialize < 'de >"
---
# NullDecimal::impl Deserialize < 'de >

透過通用 Serde visitor 反序列化。

```rust
impl< 'de > Deserialize < 'de > for NullDecimal
```

## 參數、回傳值及常規用法

deserializer: 實現 Deserializer 的 D；回傳 Self 或 D::Error。接受精確小數字串、整數 visitor 輸入和 null。拒絕浮點 visitor 輸入：JSON 小數或指數數字應使用專門的原始 JSON/config 適配器。JSON null 產生無效 nullable，內部零未初始化。空字串無效。瀏覽器靜態 deserialize(source) 適配器對完整原始 JSON 文字執行 serde_json，範圍小於任意 Rust deserializer。錯誤提供 category="serde" 和原訊息。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-nulldecimal-impl-deserialize-de" locale="zh-Hant" />

## 相關方法

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
