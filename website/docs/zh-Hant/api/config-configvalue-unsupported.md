---
locale: zh-Hant
source_revision: 2c10c2e73488c11940444f40ef8c96bd7f288472b5195e9bdeb57320696a7e16
translation_provenance: zh-Hans-terminology-reviewed
api_id: "config::ConfigValue::Unsupported"
---
# config::ConfigValue::Unsupported

選擇 Unsupported 組態輸入。

```rust
Unsupported
```

## 參數、回傳值及常規用法

明確表示不支援的組態類型，不做強制轉換。兩個 decode 函數都回傳 Unmarshal。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-unsupported" locale="zh-Hant" />

## 瀏覽器邊界

config 與 serde feature 已啟用。來源文字和位元組最多 8192 個 UTF-8 位元組；Rust 在解析前檢查小數文字和 JSON 解碼後的字串，因此跳脫不能繞過 [資源限制](../resource-policy)。128 位整數必須使用範圍內的 bigint。借用的 Decimal/NullDecimal 在解碼時必須仍有效，轉換生成獨立值，不消耗輸入。Serde 的通用 trait 語法由具體 JSON 適配器演示。

## 相關方法

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
