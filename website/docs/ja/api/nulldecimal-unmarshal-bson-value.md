---
locale: ja
source_revision: 59bfa6e69fb4720796373509ad0a7f4325fcfd6dbb9c49ba19604b5d730cd4a1
translation_provenance: en-US-japanese-reviewed
api_id: "NullDecimal::unmarshal_bson_value"
---
# NullDecimal::unmarshal_bson_value

BSON スカラーを既存の値へデコードします。

```rust
fn unmarshal_bson_value (& mut self , kind : u8 , data : & [u8]) -> Result < () , DecimalError >
```

## 引数、戻り値、通常の使い方

kind: u8/number、data: &[u8]/Uint8Array。レシーバーを変更し、`Result<(), DecimalError>`/undefined を返します。Rust は bson を必要とします。double (1)、string (2)、null (10)、int32 (16)、int64 (18)、Decimal128 (19) に対応し、数値ペイロードは BSON のリトルエンディアンバイト列です。int64 は JS number の精度損失を避けるため生のバイト列を渡してください。BSON null は valid を解除し、内部 Decimal を未初期化のゼロにリセットします。null 以外の入力の nullable デコードが成功すると valid=true にします。未対応の種類、途中で切れたデータ、非有限数では部分変更せずに失敗します。互換性上の注意として、文字列デコードは宣言された最後のバイトを NUL か確認せずに読み飛ばし、必要なペイロードを超えるバイト列は無視します。Decimal128 は Rust コアがデコードして対応する十進精度を保持しますが、marshal_bson_value の出力は引き続き文字列です。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="binary-bson-nulldecimal-unmarshal-bson-value" locale="ja" />

## 関連メソッド

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
