---
locale: ja
source_revision: 8de700fe2bea77817d4cc64ce868f77fd5301e630577d698cc1cf5a3b4f76ce5
translation_provenance: en-US-japanese-reviewed
api_id: "reexport::PRECISION_FIXED_SIZE"
---
# reexport::PRECISION_FIXED_SIZE

バイナリの精度接頭部のバイト長を読み取ります。

```rust
pub use binary::PRECISION_FIXED_SIZE as PRECISION_FIXED_SIZE
```

## 引数、戻り値、通常の使い方

値が 4 の Rust usize 定数です。Go 互換のバイナリ通信形式の、ビッグエンディアンのスケール接頭部のバイト長を表します。十進桁数や精度の上限ではありません。ブラウザーは Rust 由来の読み取り専用 number を公開します。クレートルートからの再エクスポートは Rust のインポート構文です。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="binary-bson-reexport-precision-fixed-size" locale="ja" />

## 関連メソッド

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
