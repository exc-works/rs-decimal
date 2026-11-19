---
locale: zh-Hant
source_revision: 76052d80c386b89ac2428898d57b9ce525f22161fd5cd3ee7956cceafe3b11e9
translation_provenance: zh-Hans-terminology-reviewed
api_id: "Decimal::marshal_json"
---
# Decimal::marshal_json

將 Decimal 編碼為 json 位元組。

```rust
fn marshal_json (& self) -> Vec < u8 >
```

## 參數、回傳值及常規用法

無參數，回傳 `Vec<u8>`/Uint8Array，內容為JSON 位元組。Rust 需要 json feature，瀏覽器建置已啟用。已初始化的 Decimal 編碼為帶引號的規範字串；預設未初始化零編碼為 JSON null。回傳位元組是獨立副本，讀取不改變狀態。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-decimal-marshal-json" locale="zh-Hant" />

## 相關方法

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
