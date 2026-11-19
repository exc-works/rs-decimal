---
locale: zh-Hant
source_revision: dd4986a8dd71923263f954c74b5685b1dfaa3a4b594ef1aadb36c8a62519bd62
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::append_text
---
# Decimal::append_text

向現有可變緩衝區追加十進位 ASCII 位元組。

```rust
fn append_text (& self , dst : & mut Vec < u8 >)
```

## 參數、回傳值及常規用法

`dst: &mut Vec<u8>`，以 ByteBuffer 物件表示。Rust 回傳 ()，JS 回傳 undefined。保留已有位元組，原地修改目標。移除小數末尾零。Decimal 和緩衝區都仍可使用。ByteBuffer.new 複製 Uint8Array；to_uint8_array 回傳副本，不是 Wasm 記憶體視圖。資源錯誤發生在修改前，保留原緩衝區。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-decimal-append-text" locale="zh-Hant" />

## 相關方法

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
