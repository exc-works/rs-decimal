---
locale: zh-Hant
source_revision: dabe3e417d155ec62acc296ebb36a3f87bdc044baf1a1ffd4eb27cbaf056b77d
translation_provenance: zh-Hans-terminology-reviewed
api_id: "Decimal::marshal_text"
---
# Decimal::marshal_text

將 Decimal 編碼為 text 位元組。

```rust
fn marshal_text (& self) -> Vec < u8 >
```

## 參數、回傳值及常規用法

無參數，回傳 `Vec<u8>`/Uint8Array，內容為規範的十進位 ASCII 文字。移除小數末尾零。預設 Decimal 零輸出 ASCII 0。回傳位元組是獨立副本，讀取不改變狀態。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-decimal-marshal-text" locale="zh-Hant" />

## 相關方法

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
