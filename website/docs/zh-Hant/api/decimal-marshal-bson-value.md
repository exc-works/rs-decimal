---
locale: zh-Hant
source_revision: 44afbeff0d59cbde33e83e7f33c7e22c9903950f7ecf43d6f8230c6cde4b2ec4
translation_provenance: zh-Hans-terminology-reviewed
api_id: "Decimal::marshal_bson_value"
---
# Decimal::marshal_bson_value

編碼 BSON 類型位元組和純量負載。

```rust
fn marshal_bson_value (& self) -> Result < (u8 , Vec < u8 >) , DecimalError >
```

## 參數、回傳值及常規用法

無參數，回傳 `Result<(u8, Vec<u8>), DecimalError>`/[number, Uint8Array]。Rust 需要 bson feature，瀏覽器建置已啟用。元組包含 BSON 純量，而非完整 BSON 文件。已初始化值編碼為 BSON 字串（類型 2），不是 Decimal128；未初始化零編碼為類型 10 和空負載。字串負載先寫小端 i32 位元組長度（包含結尾 NUL），隨後寫無小數末尾零的規範文字和 NUL。回傳位元組是獨立副本；編碼不改變初始化或有效旗標。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="binary-bson-decimal-marshal-bson-value" locale="zh-Hant" />

## 相關方法

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
