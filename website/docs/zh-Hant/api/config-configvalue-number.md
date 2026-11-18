---
locale: zh-Hant
source_revision: 74e7f2ffd43f198c10e7ebd972920c487311d6c7ab45db8c6281425ea90f9c14
translation_provenance: zh-Hans-terminology-reviewed
api_id: "config::ConfigValue::Number"
---
# config::ConfigValue::Number

選擇 Number 組態輸入。

```rust
Number (& 'a str)
```

## 參數、回傳值及常規用法

傳遞數字來源文字，不經過浮點轉換。核心解析小數語法，此變體不是完整 JSON 語法驗證器。空 Number 對 nullable 目標也無效。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-number" locale="zh-Hant" />

## 瀏覽器邊界

config 與 serde feature 已啟用。來源文字和位元組最多 8192 個 UTF-8 位元組；Rust 在解析前檢查小數文字和 JSON 解碼後的字串，因此跳脫不能繞過 [資源限制](../resource-policy)。128 位整數必須使用範圍內的 bigint。借用的 Decimal/NullDecimal 在解碼時必須仍有效，轉換生成獨立值，不消耗輸入。Serde 的通用 trait 語法由具體 JSON 適配器演示。

## 相關方法

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
