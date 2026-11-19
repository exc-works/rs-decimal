---
locale: zh-Hant
source_revision: 0268b5969d6d0344c17f1c1c3dee925ce05b68c8ea0fc3782a3dde3b820e544c
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

## 相關方法

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
