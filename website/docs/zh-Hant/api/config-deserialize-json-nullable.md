---
locale: zh-Hant
source_revision: 79f19f9f6f20dd385b69becd2b7e8e1abfeb536b960dc7d8b240d8301a6064de
translation_provenance: zh-Hans-terminology-reviewed
api_id: "config::deserialize_json_nullable"
---
# config::deserialize_json_nullable

反序列化精確 JSON 組態欄位。

```rust
fn deserialize_json_nullable < 'de , D : Deserializer < 'de > > (deserializer : D ,) -> Result < NullDecimal , D :: Error >
```

## 參數、回傳值及常規用法

deserializer: 實現 Serde Deserializer 的 D；用於 #[serde(deserialize_with = "rs_decimal::config::deserialize_json_nullable")]. 需要 serde_json 源和 config feature。保留原始數字 token，不經過浮點數。瀏覽器提供具體適配器，參數為一個完整 JSON 純量來源字串，不是任意 deserializer。JSON null 和恰好為空的 JSON 字串變為無效值；只有空白的字串仍失敗。成功的非空值變為有效。錯誤類型為 Serde deserializer 的錯誤；瀏覽器使用 category="serde" 保留訊息，不虛構 DecimalError 類別。拒絕末尾非空白內容。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-deserialize-json-nullable" locale="zh-Hant" />

## 相關方法

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
