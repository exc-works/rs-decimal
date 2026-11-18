---
locale: ja
source_revision: d77e5dd32cf21e2ddfd9d7df9654834b1090bc84b953f82d973c756dd33fd206
translation_provenance: en-US-japanese-reviewed
api_id: "Decimal::unmarshal_bson_value"
---
# Decimal::unmarshal_bson_value

BSON スカラーを既存の値へデコードします。

```rust
fn unmarshal_bson_value (& mut self , kind : u8 , data : & [u8]) -> Result < () , DecimalError >
```

## 引数、戻り値、通常の使い方

kind: u8/number、data: &[u8]/Uint8Array。レシーバーを変更し、`Result<(), DecimalError>`/undefined を返します。Rust は bson を必要とします。double (1)、string (2)、null (10)、int32 (16)、int64 (18)、Decimal128 (19) に対応し、数値ペイロードは BSON のリトルエンディアンバイト列です。int64 は JS number の精度損失を避けるため生のバイト列を渡してください。BSON null はレシーバーを未初期化のゼロにリセットします。変更しない unmarshal_json(null) とは異なります。null 以外の入力の nullable デコードが成功すると valid=true にします。未対応の種類、途中で切れたデータ、非有限数では部分変更せずに失敗します。互換性上の注意として、文字列デコードは宣言された最後のバイトを NUL か確認せずに読み飛ばし、必要なペイロードを超えるバイト列は無視します。Decimal128 は Rust コアがデコードして対応する十進精度を保持しますが、marshal_bson_value の出力は引き続き文字列です。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="binary-bson-decimal-unmarshal-bson-value" locale="ja" />

## ブラウザーの制約

入力は Uint8Array、kind は 0–255 の整数である必要があります。Wasm へのコピー前に入力を 65536 バイトに制限します。Rust はバイナリのスケールを最大 1000、実際の係数サイズを最大 16384 ビットに制限します。BSON 文字列はテキストの 8192 バイト上限に従い、Decimal128 の指数展開も[リソース検査](../resource-policy)を受けます。これらの制限はネイティブライブラリより狭い場合があります。検査はレシーバーの変更前に行い、コアとリソースのエラーを区別します。ムーブ済みのハンドルは再利用できません。

## 関連メソッド

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
