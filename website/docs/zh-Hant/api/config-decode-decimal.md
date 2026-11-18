---
locale: zh-Hant
source_revision: 9abf8678e26851a7c2e2283fba52cbf7f426bb86feeae64e6b8312a6a7254ba7
translation_provenance: zh-Hans-terminology-reviewed
api_id: "config::decode_decimal"
---
# config::decode_decimal

解碼明確類型的組態值。

```rust
fn decode_decimal (value : ConfigValue < '_ >) -> Result < Decimal , DecimalError >
```

## 參數、回傳值及常規用法

value: ConfigValue；回傳 `Result<Decimal, DecimalError>`/新的 Decimal 控制代碼。Rust 需要 config feature。不可空解碼拒絕 Null 和無效 Nullable，不自動補零。現有 Decimal 和有效 Nullable 按完整表示複製，包括未初始化零。Text/Number/Bytes 精確解析小數文字，不移除 SQL 引號。浮點數已帶二進位捨入；保留原始數字時用文字或 bigint。瀏覽器變體屬於本次執行，借用控制代碼或緩衝區至解碼呼叫。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-decode-decimal" locale="zh-Hant" />

## 瀏覽器邊界

config 與 serde feature 已啟用。來源文字和位元組最多 8192 個 UTF-8 位元組；Rust 在解析前檢查小數文字和 JSON 解碼後的字串，因此跳脫不能繞過 [資源限制](../resource-policy)。128 位整數必須使用範圍內的 bigint。借用的 Decimal/NullDecimal 在解碼時必須仍有效，轉換生成獨立值，不消耗輸入。Serde 的通用 trait 語法由具體 JSON 適配器演示。

## 相關方法

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
