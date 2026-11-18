---
locale: zh-Hant
source_revision: 3576d2b52e6de01c1594ed23748d6a482f290fdb6f5b45fc15ec07a4cf297d34
translation_provenance: zh-Hans-terminology-reviewed
api_id: "config::ConfigValue::Bytes"
---
# config::ConfigValue::Bytes

選擇 Bytes 組態輸入。

```rust
Bytes (& 'a [u8])
```

## 參數、回傳值及常規用法

借用小數文字位元組，不是二進位小數負載。瀏覽器觀察解碼前的 Uint8Array 修改，解碼不修改陣列。無效 UTF-8 導致小數解析失敗。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-bytes" locale="zh-Hant" />

## 瀏覽器邊界

config 與 serde feature 已啟用。來源文字和位元組最多 8192 個 UTF-8 位元組；Rust 在解析前檢查小數文字和 JSON 解碼後的字串，因此跳脫不能繞過 [資源限制](../resource-policy)。128 位整數必須使用範圍內的 bigint。借用的 Decimal/NullDecimal 在解碼時必須仍有效，轉換生成獨立值，不消耗輸入。Serde 的通用 trait 語法由具體 JSON 適配器演示。

## 相關方法

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
