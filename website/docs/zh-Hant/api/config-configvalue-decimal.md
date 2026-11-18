---
locale: zh-Hant
source_revision: b283786af672117d0c3017e513f97f5df201a379427acab824b383812aa4cf93
translation_provenance: zh-Hans-terminology-reviewed
api_id: "config::ConfigValue::Decimal"
---
# config::ConfigValue::Decimal

選擇 Decimal 組態輸入。

```rust
Decimal (& 'a Decimal)
```

## 參數、回傳值及常規用法

借用現有小數，在解碼時複製係數、scale 和初始化狀態。來源值仍可使用。瀏覽器來源值在解碼前釋放或移動後，該借用變體不可用。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-decimal" locale="zh-Hant" />

## 瀏覽器邊界

config 與 serde feature 已啟用。來源文字和位元組最多 8192 個 UTF-8 位元組；Rust 在解析前檢查小數文字和 JSON 解碼後的字串，因此跳脫不能繞過 [資源限制](../resource-policy)。128 位整數必須使用範圍內的 bigint。借用的 Decimal/NullDecimal 在解碼時必須仍有效，轉換生成獨立值，不消耗輸入。Serde 的通用 trait 語法由具體 JSON 適配器演示。

## 相關方法

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
