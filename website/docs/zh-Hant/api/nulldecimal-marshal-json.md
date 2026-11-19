---
locale: zh-Hant
source_revision: f90f9a719348f32512d576fc2ef23076c6492acd11b0dbe87ae7f8c72a7c2eab
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

## 相關方法

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
