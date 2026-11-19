---
locale: ja
source_revision: 8f3a1f2c3a3bbab8fcd006696e4bbfc5801d795ba8c28546dfa164fc41913e69
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::append_text_with_trailing_zeros
---
# Decimal::append_text_with_trailing_zeros

既存の可変バッファーに小数の ASCII バイト列を追記します。

```rust
fn append_text_with_trailing_zeros (& self , dst : & mut Vec < u8 >)
```

## 引数、戻り値、通常の使い方

`dst: &mut Vec<u8>` を ByteBuffer ラッパーで表します。Rust は ()、JS は undefined を返します。既存のバイト列を保持し、宛先をその場で変更します。保存された小数部分のゼロをすべて保持します。Decimal とバッファーは引き続き使用できます。Uint8Array は ByteBuffer.new にコピーされ、to_uint8_array は Wasm メモリービューではなくコピーを返します。リソースの失敗は変更前に起こり、バッファーは変わりません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-decimal-append-text-with-trailing-zeros" locale="ja" />

## 関連メソッド

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
