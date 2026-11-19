---
locale: ja
source_revision: 65435d6f54d8777702cf2f629258b85e312b71241d8a0725f7c209d6e7b4a200
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

## 関連メソッド

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
