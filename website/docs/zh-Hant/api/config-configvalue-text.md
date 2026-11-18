---
locale: zh-Hant
source_revision: 9cc3411fb84577d5ea175fe7b4b5d7e0d13f86f3ba54a79c3a0728b721c31211
translation_provenance: zh-Hans-terminology-reviewed
api_id: "config::ConfigValue::Text"
---
# config::ConfigValue::Text

選擇 Text 組態輸入。

```rust
Text (& 'a str)
```

## 參數、回傳值及常規用法

借用精確小數文字。首尾空白由小數核心解析，不移除 SQL 風格外圍引號。僅恰好為空的文字在 nullable 解碼時表示缺失。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-text" locale="zh-Hant" />

## 瀏覽器邊界

config 與 serde feature 已啟用。來源文字和位元組最多 8192 個 UTF-8 位元組；Rust 在解析前檢查小數文字和 JSON 解碼後的字串，因此跳脫不能繞過 [資源限制](../resource-policy)。128 位整數必須使用範圍內的 bigint。借用的 Decimal/NullDecimal 在解碼時必須仍有效，轉換生成獨立值，不消耗輸入。Serde 的通用 trait 語法由具體 JSON 適配器演示。

## 相關方法

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
