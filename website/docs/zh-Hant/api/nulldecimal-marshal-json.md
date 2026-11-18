---
locale: zh-Hant
source_revision: ce07dfd04d96401a00570a69b98dc03abf2e70bda05348ea2e8e6bff0225c443
translation_provenance: zh-Hans-terminology-reviewed
api_id: "NullDecimal::marshal_json"
---
# NullDecimal::marshal_json

將 NullDecimal 編碼為 json 位元組。

```rust
fn marshal_json (& self) -> Vec < u8 >
```

## 參數、回傳值及常規用法

無參數，回傳 `Vec<u8>`/Uint8Array，內容為JSON 位元組。Rust 需要 json feature，瀏覽器建置已啟用。無效 nullable 編碼為 JSON null；有效值編碼為帶引號的規範小數字串，即使內部 Decimal 未初始化。回傳位元組是獨立副本，讀取不改變狀態。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-nulldecimal-marshal-json" locale="zh-Hant" />

## 瀏覽器邊界

解碼輸入最多 8192 位元組，參數字串還在複製到 Wasm 前檢查長度。Rust 對文字及 JSON 解碼後的字串再次檢查 [十進位資源限制](../resource-policy)，因此跳脫字元不能繞過指數展開或精度限制。無效 UTF-8 和 JSON/小數語法仍由核心報告。輸入錯誤和資源錯誤都不會部分更新接收者；已移動的物件不可再讀寫。

## 相關方法

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
