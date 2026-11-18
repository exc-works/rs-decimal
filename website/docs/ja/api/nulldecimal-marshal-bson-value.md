---
locale: ja
source_revision: b07552403b149d5a66b24c1aa4c0547e8259f4d124ac401ca1161c70929a6aa6
translation_provenance: en-US-japanese-reviewed
api_id: "NullDecimal::marshal_bson_value"
---
# NullDecimal::marshal_bson_value

BSON の型バイトとスカラーペイロードをエンコードします。

```rust
fn marshal_bson_value (& self) -> Result < (u8 , Vec < u8 >) , DecimalError >
```

## 引数、戻り値、通常の使い方

引数なし。`Result<(u8, Vec<u8>), DecimalError>`/[number, Uint8Array] を返します。Rust は bson 機能を必要とし、ブラウザービルドでは有効です。タプルは完全な BSON 文書ではなく、生の BSON スカラーを含みます。有効な nullable は内部が未初期化のゼロでも BSON 文字列になります。無効な値は型 10 と空のペイロードになります。文字列ペイロードは末尾の NUL を含むリトルエンディアン i32 のバイト長、その後に小数部分の末尾のゼロを除去した標準テキストが続きます。返されたペイロードは独立したコピーで、エンコードは初期化状態や有効性を変更しません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="binary-bson-nulldecimal-marshal-bson-value" locale="ja" />

## ブラウザーの制約

入力は Uint8Array、kind は 0–255 の整数である必要があります。Wasm へのコピー前に入力を 65536 バイトに制限します。Rust はバイナリのスケールを最大 1000、実際の係数サイズを最大 16384 ビットに制限します。BSON 文字列はテキストの 8192 バイト上限に従い、Decimal128 の指数展開も[リソース検査](../resource-policy)を受けます。これらの制限はネイティブライブラリより狭い場合があります。検査はレシーバーの変更前に行い、コアとリソースのエラーを区別します。ムーブ済みのハンドルは再利用できません。

## 関連メソッド

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
