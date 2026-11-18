---
locale: zh-Hant
source_revision: 4824e4dc634e3ef762c96aa330a4aec14949a376f5c5d2b586754496c1c0639f
translation_provenance: zh-Hans-terminology-reviewed
api_id: "NullDecimal::impl Deserialize < 'de >::deserialize"
---
# NullDecimal::impl Deserialize < 'de >::deserialize

透過通用 Serde visitor 反序列化。

```rust
fn deserialize < D : Deserializer < 'de > > (d : D) -> Result < Self , D :: Error >
```

## 參數、回傳值及常規用法

deserializer: 實現 Deserializer 的 D；回傳 Self 或 D::Error。接受精確小數字串、整數 visitor 輸入和 null。拒絕浮點 visitor 輸入：JSON 小數或指數數字應使用專門的原始 JSON/config 適配器。JSON null 產生無效 nullable，內部零未初始化。空字串無效。瀏覽器靜態 deserialize(source) 適配器對完整原始 JSON 文字執行 serde_json，範圍小於任意 Rust deserializer。錯誤提供 category="serde" 和原訊息。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-nulldecimal-impl-deserialize-de-deserialize" locale="zh-Hant" />

## 瀏覽器邊界

config 與 serde feature 已啟用。來源文字和位元組最多 8192 個 UTF-8 位元組；Rust 在解析前檢查小數文字和 JSON 解碼後的字串，因此跳脫不能繞過 [資源限制](../resource-policy)。128 位整數必須使用範圍內的 bigint。借用的 Decimal/NullDecimal 在解碼時必須仍有效，轉換生成獨立值，不消耗輸入。Serde 的通用 trait 語法由具體 JSON 適配器演示。

## 相關方法

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
