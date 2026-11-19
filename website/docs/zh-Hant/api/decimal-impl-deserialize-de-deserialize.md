---
locale: zh-Hant
source_revision: 24e00c21cccbe1d37441ecf7320c640c608396a025aad09f2a518e2a7e914703
translation_provenance: zh-Hans-terminology-reviewed
api_id: "Decimal::impl Deserialize < 'de >::deserialize"
---
# Decimal::impl Deserialize < 'de >::deserialize

透過通用 Serde visitor 反序列化。

```rust
fn deserialize < D : Deserializer < 'de > > (d : D) -> Result < Self , D :: Error >
```

## 參數、回傳值及常規用法

deserializer: 實現 Deserializer 的 D；回傳 Self 或 D::Error。接受精確小數字串、整數 visitor 輸入和 null。拒絕浮點 visitor 輸入：JSON 小數或指數數字應使用專門的原始 JSON/config 適配器。JSON null 產生未初始化的 Decimal 零。空字串無效。瀏覽器靜態 deserialize(source) 適配器對完整原始 JSON 文字執行 serde_json，範圍小於任意 Rust deserializer。錯誤提供 category="serde" 和原訊息。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-decimal-impl-deserialize-de-deserialize" locale="zh-Hant" />

## 相關方法

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
