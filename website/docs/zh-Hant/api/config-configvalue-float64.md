---
locale: zh-Hant
source_revision: 21a2321ebb07dad0e89ac689f0921883a544f6575bbd19e235e51c9aee9deb73
translation_provenance: zh-Hans-terminology-reviewed
api_id: "config::ConfigValue::Float64"
---
# config::ConfigValue::Float64

選擇 Float64 組態輸入。

```rust
Float64 (f64)
```

## 參數、回傳值及常規用法

透過核心最短十進位轉換傳遞 binary64。非有限數回傳 Unmarshal，已因浮點捨入丟失的數字無法恢復。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-float64" locale="zh-Hant" />

## 瀏覽器邊界

config 與 serde feature 已啟用。來源文字和位元組最多 8192 個 UTF-8 位元組；Rust 在解析前檢查小數文字和 JSON 解碼後的字串，因此跳脫不能繞過 [資源限制](../resource-policy)。128 位整數必須使用範圍內的 bigint。借用的 Decimal/NullDecimal 在解碼時必須仍有效，轉換生成獨立值，不消耗輸入。Serde 的通用 trait 語法由具體 JSON 適配器演示。

## 相關方法

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
