---
locale: zh-Hant
source_revision: 5bb3ef8c2d718846f6ac7a4004c6720c37095a64615189ff33906f2fba479837
translation_provenance: zh-Hans-terminology-reviewed
api_id: "config::ConfigValue::Signed"
---
# config::ConfigValue::Signed

選擇 Signed 組態輸入。

```rust
Signed (i128)
```

## 參數、回傳值及常規用法

傳遞精確有符號 i128。瀏覽器要求 [-2^127, 2^127-1] 範圍的 bigint；轉換前拒絕 JS number 和越界值。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-signed" locale="zh-Hant" />

## 瀏覽器邊界

config 與 serde feature 已啟用。來源文字和位元組最多 8192 個 UTF-8 位元組；Rust 在解析前檢查小數文字和 JSON 解碼後的字串，因此跳脫不能繞過 [資源限制](../resource-policy)。128 位整數必須使用範圍內的 bigint。借用的 Decimal/NullDecimal 在解碼時必須仍有效，轉換生成獨立值，不消耗輸入。Serde 的通用 trait 語法由具體 JSON 適配器演示。

## 相關方法

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
