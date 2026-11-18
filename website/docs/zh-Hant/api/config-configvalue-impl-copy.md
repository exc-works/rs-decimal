---
locale: zh-Hant
source_revision: 178894cd0af1970f19442cd816169372cdb2496adc78c87aaec82dd961b0da2e
translation_provenance: zh-Hans-terminology-reviewed
api_id: "config::ConfigValue::impl Copy"
---
# config::ConfigValue::impl Copy

使用原生 Copy 實現。

```rust
impl Copy for config::ConfigValue
```

## 參數、回傳值及常規用法

ConfigValue 是小型類型列舉。Clone/Copy 保留借用參照，不深拷貝參照目標。Debug 是診斷文字，不是穩定的傳輸格式。原生 Rust trait 語法單獨測試，不是瀏覽器方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-impl-copy" locale="zh-Hant" />

## 瀏覽器邊界

config 與 serde feature 已啟用。來源文字和位元組最多 8192 個 UTF-8 位元組；Rust 在解析前檢查小數文字和 JSON 解碼後的字串，因此跳脫不能繞過 [資源限制](../resource-policy)。128 位整數必須使用範圍內的 bigint。借用的 Decimal/NullDecimal 在解碼時必須仍有效，轉換生成獨立值，不消耗輸入。Serde 的通用 trait 語法由具體 JSON 適配器演示。

## 相關方法

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
