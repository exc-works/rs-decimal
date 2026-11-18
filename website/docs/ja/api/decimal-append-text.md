---
locale: ja
source_revision: bf26b0f6b02acd5afbbeb54a43a96410bc32ae1ee6825c86870266f2a294720f
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

## ブラウザーの制約

[リソースポリシー](../resource-policy)は書式幅を 65536 バイト、精度を 1000 に制限し、固定小数点の書式では係数増大も確認します。バッファーは 65536 バイトまでです。追記は保守的なサイズ見積もりを使うため、実際にはちょうど収まる操作も拒否する場合があります。リソースエラーは入力やバッファーを変更しません。print は別途長い表示テキストを切り詰めます。全結果を調べるには長さを確認するか、小さな部分に分けて出力してください。

## 関連メソッド

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
