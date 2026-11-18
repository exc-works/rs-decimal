---
locale: ja
source_revision: b225904d6e477d16ac1df6ba10bcfee3249227a343581851ba1cc46cc84f5863
translation_provenance: en-US-japanese-reviewed
api_id: "Decimal::marshal_binary"
---
# Decimal::marshal_binary

Go 互換の precision/Gob 通信形式へエンコードします。

```rust
fn marshal_binary (& self) -> Result < Vec < u8 > , DecimalError >
```

## 引数、戻り値、通常の使い方

引数なし。`Result<Vec<u8>, DecimalError>`/Uint8Array を返します。初期化済みの値では小数部分の末尾のゼロを除去してから、4 バイトのビッグエンディアンのスケール、big.Int Gob のバージョン／符号バイト、ビッグエンディアンの絶対値を書き込みます。これは小数スカラーのペイロードであり、IEEE 浮動小数点ではありません。未初期化のゼロは空のバイト列、初期化済みのゼロは [0,0,0,0,2] になります。返されたコピーを変更しても Decimal は変わりません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="binary-bson-decimal-marshal-binary" locale="ja" />

## ブラウザーの制約

入力は Uint8Array、kind は 0–255 の整数である必要があります。Wasm へのコピー前に入力を 65536 バイトに制限します。Rust はバイナリのスケールを最大 1000、実際の係数サイズを最大 16384 ビットに制限します。BSON 文字列はテキストの 8192 バイト上限に従い、Decimal128 の指数展開も[リソース検査](../resource-policy)を受けます。これらの制限はネイティブライブラリより狭い場合があります。検査はレシーバーの変更前に行い、コアとリソースのエラーを区別します。ムーブ済みのハンドルは再利用できません。

## 関連メソッド

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
