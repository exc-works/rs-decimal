---
locale: ja
source_revision: 9fa4dc63685c8b7863c54238c3b1bc5377c717d45438360151595cb826deb45d
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

## 関連メソッド

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
