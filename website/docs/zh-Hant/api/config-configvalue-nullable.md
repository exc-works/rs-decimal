---
locale: zh-Hant
source_revision: 4748c543f00fa16cc184b2f3ac3218ce12c09982f49453b64f19d87393fb722a
translation_provenance: zh-Hans-terminology-reviewed
api_id: "config::ConfigValue::Nullable"
---
# config::ConfigValue::Nullable

選擇 Nullable 組態輸入。

```rust
Nullable (& 'a NullDecimal)
```

## 參數、回傳值及常規用法

借用現有 nullable。decode_nullable 複製有效旗標和隱藏資料；decode_decimal 只接受有效值並複製內部 Decimal。解碼前的後續修改會被觀察到。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-nullable" locale="zh-Hant" />

## 瀏覽器邊界

config 與 serde feature 已啟用。來源文字和位元組最多 8192 個 UTF-8 位元組；Rust 在解析前檢查小數文字和 JSON 解碼後的字串，因此跳脫不能繞過 [資源限制](../resource-policy)。128 位整數必須使用範圍內的 bigint。借用的 Decimal/NullDecimal 在解碼時必須仍有效，轉換生成獨立值，不消耗輸入。Serde 的通用 trait 語法由具體 JSON 適配器演示。

## 相關方法

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
