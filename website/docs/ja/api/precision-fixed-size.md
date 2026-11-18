---
locale: ja
source_revision: 73595bceae9a4435e67075be3cf3874619847ef2c8e049d0faeae85eae4feb26
translation_provenance: en-US-japanese-reviewed
api_id: "PRECISION_FIXED_SIZE"
---
# PRECISION_FIXED_SIZE

バイナリの精度接頭部のバイト長を読み取ります。

```rust
const PRECISION_FIXED_SIZE: usize = 4
```

## 引数、戻り値、通常の使い方

値が 4 の Rust usize 定数です。Go 互換のバイナリ通信形式の、ビッグエンディアンのスケール接頭部のバイト長を表します。十進桁数や精度の上限ではありません。ブラウザーは Rust 由来の読み取り専用 number を公開します。クレートルートからの再エクスポートは Rust のインポート構文です。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="binary-bson-precision-fixed-size" locale="ja" />

## ブラウザーの制約

入力は Uint8Array、kind は 0–255 の整数である必要があります。Wasm へのコピー前に入力を 65536 バイトに制限します。Rust はバイナリのスケールを最大 1000、実際の係数サイズを最大 16384 ビットに制限します。BSON 文字列はテキストの 8192 バイト上限に従い、Decimal128 の指数展開も[リソース検査](../resource-policy)を受けます。これらの制限はネイティブライブラリより狭い場合があります。検査はレシーバーの変更前に行い、コアとリソースのエラーを区別します。ムーブ済みのハンドルは再利用できません。

## 関連メソッド

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
