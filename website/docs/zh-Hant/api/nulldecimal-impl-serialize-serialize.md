---
locale: zh-Hant
source_revision: 67d49a92b2e2ca62692f06791c5907e6136e1bf8593cdb08edcf4fb008700159
translation_provenance: zh-Hans-terminology-reviewed
api_id: "NullDecimal::impl Serialize::serialize"
---
# NullDecimal::impl Serialize::serialize

透過 Rust Serde trait 序列化。

```rust
fn serialize < S : Serializer > (& self , s : S) -> Result < S :: Ok , S :: Error >
```

## 參數、回傳值及常規用法

serializer: 實現 Serializer 的 S；回傳 S::Ok/S::Error。核心序列化為規範小數字串或 null。無效 nullable 序列化為 null；內部零未初始化的有效值序列化為字串 "0"。移除小數末尾零。瀏覽器 serialize() 適配器選擇 serde_json，回傳完整 JSON 文字，不暴露任意 Rust serializer。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-nulldecimal-impl-serialize-serialize" locale="zh-Hant" />

## 瀏覽器邊界

config 與 serde feature 已啟用。來源文字和位元組最多 8192 個 UTF-8 位元組；Rust 在解析前檢查小數文字和 JSON 解碼後的字串，因此跳脫不能繞過 [資源限制](../resource-policy)。128 位整數必須使用範圍內的 bigint。借用的 Decimal/NullDecimal 在解碼時必須仍有效，轉換生成獨立值，不消耗輸入。Serde 的通用 trait 語法由具體 JSON 適配器演示。

## 相關方法

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
