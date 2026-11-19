---
locale: zh-Hant
source_revision: 8f3a1f2c3a3bbab8fcd006696e4bbfc5801d795ba8c28546dfa164fc41913e69
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::append_text_with_trailing_zeros
---
# Decimal::append_text_with_trailing_zeros

向現有可變緩衝區追加十進位 ASCII 位元組。

```rust
fn append_text_with_trailing_zeros (& self , dst : & mut Vec < u8 >)
```

## 參數、回傳值及常規用法

`dst: &mut Vec<u8>`，以 ByteBuffer 物件表示。Rust 回傳 ()，JS 回傳 undefined。保留已有位元組，原地修改目標。保留所有儲存的小數末尾零。Decimal 和緩衝區都仍可使用。ByteBuffer.new 複製 Uint8Array；to_uint8_array 回傳副本，不是 Wasm 記憶體視圖。資源錯誤發生在修改前，保留原緩衝區。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-decimal-append-text-with-trailing-zeros" locale="zh-Hant" />

## 相關方法

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
