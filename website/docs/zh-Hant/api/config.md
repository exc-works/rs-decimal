---
locale: zh-Hant
source_revision: 03da42d48c5d8b250daaaa93d59162466d4ca8b88da11e32c7b577b2d36d0747
translation_provenance: zh-Hans-terminology-reviewed
api_id: "config"
---
# config

解碼明確類型的組態值。

```rust
pub mod config
```

## 參數、回傳值及常規用法

此組態適配器支援文字/文字位元組、精確數字文字、有符號或無符號 128 位整數、浮點數以及借用的 Decimal/NullDecimal。Rust 需要 config feature。不可空解碼拒絕 Null 和無效 Nullable，不自動補零。現有 Decimal 和有效 Nullable 按完整表示複製，包括未初始化零。Text/Number/Bytes 精確解析小數文字，不移除 SQL 引號。浮點數已帶二進位捨入；保留原始數字時用文字或 bigint。瀏覽器變體屬於本次執行，借用控制代碼或緩衝區至解碼呼叫。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config" locale="zh-Hant" />

## 相關方法

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
