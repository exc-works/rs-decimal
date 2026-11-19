---
locale: ja
source_revision: dd4986a8dd71923263f954c74b5685b1dfaa3a4b594ef1aadb36c8a62519bd62
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::append_text
---
# Decimal::append_text

既存の可変バッファーに小数の ASCII バイト列を追記します。

```rust
fn append_text (& self , dst : & mut Vec < u8 >)
```

## 引数、戻り値、通常の使い方

`dst: &mut Vec<u8>` を ByteBuffer ラッパーで表します。Rust は ()、JS は undefined を返します。既存のバイト列を保持し、宛先をその場で変更します。小数部分の末尾のゼロを除去します。Decimal とバッファーは引き続き使用できます。Uint8Array は ByteBuffer.new にコピーされ、to_uint8_array は Wasm メモリービューではなくコピーを返します。リソースの失敗は変更前に起こり、バッファーは変わりません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-decimal-append-text" locale="ja" />

## 関連メソッド

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
