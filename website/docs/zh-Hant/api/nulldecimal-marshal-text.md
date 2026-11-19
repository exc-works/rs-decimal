---
locale: zh-Hant
source_revision: 2b341a7995c77dbb72c2087390bcd56b4208d2c9ec45de8ea22c2ce3f209501a
translation_provenance: zh-Hans-terminology-reviewed
api_id: "NullDecimal::marshal_text"
---
# NullDecimal::marshal_text

將 NullDecimal 編碼為 text 位元組。

```rust
fn marshal_text (& self) -> Vec < u8 >
```

## 參數、回傳值及常規用法

無參數，回傳 `Vec<u8>`/Uint8Array，內容為規範的十進位 ASCII 文字。有效值編碼為規範小數文字；無效值編碼為空位元組，不是單詞 null。回傳位元組是獨立副本，讀取不改變狀態。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-nulldecimal-marshal-text" locale="zh-Hant" />

## 相關方法

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
