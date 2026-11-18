---
locale: zh-Hant
source_revision: 77287e6c4550f824e46a4556e0aaf001ccf92f4e78984f337df5b480f5f48874
translation_provenance: zh-Hans-terminology-reviewed
api_id: "config::decode_nullable"
---
# config::decode_nullable

解碼可空組態值。

```rust
fn decode_nullable (value : ConfigValue < '_ >) -> Result < NullDecimal , DecimalError >
```

## 參數、回傳值及常規用法

value: ConfigValue；回傳 `Result<NullDecimal, DecimalError>`/新的可空控制代碼。Null、恰好為空的 Text 和空 Bytes 產生無效值，內部零未初始化。只有空白的 Text 和空 Number 仍會解析失敗。現有 Nullable 複製有效旗標和隱藏內部資料，即使原值無效；借用 Decimal 時結果有效，但不會強制初始化。其他輸入遵循 decode_decimal，成功後變為有效。輸入不被消耗。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-decode-nullable" locale="zh-Hant" />

## 瀏覽器邊界

config 與 serde feature 已啟用。來源文字和位元組最多 8192 個 UTF-8 位元組；Rust 在解析前檢查小數文字和 JSON 解碼後的字串，因此跳脫不能繞過 [資源限制](../resource-policy)。128 位整數必須使用範圍內的 bigint。借用的 Decimal/NullDecimal 在解碼時必須仍有效，轉換生成獨立值，不消耗輸入。Serde 的通用 trait 語法由具體 JSON 適配器演示。

## 相關方法

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
