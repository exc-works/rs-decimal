---
locale: zh-Hant
source_revision: d7c6490115eacc09ab0edd5dff3653c7f3971bc72919c598a24d41b9dd4bd07a
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

## 瀏覽器邊界

解碼輸入最多 8192 位元組，參數字串還在複製到 Wasm 前檢查長度。Rust 對文字及 JSON 解碼後的字串再次檢查 [十進位資源限制](../resource-policy)，因此跳脫字元不能繞過指數展開或精度限制。無效 UTF-8 和 JSON/小數語法仍由核心報告。輸入錯誤和資源錯誤都不會部分更新接收者；已移動的物件不可再讀寫。

## 相關方法

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
