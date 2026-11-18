---
locale: ja
source_revision: 0dd0f243213461bb54e73305b48db01cf1b491f68e6951552c4179ef98edf6bf
translation_provenance: en-US-japanese-reviewed
api_id: "reexport::DecimalError"
---
# reexport::DecimalError

クレートルートから DecimalError をインポートします。

```rust
pub use error::DecimalError as DecimalError
```

## 引数、戻り値、通常の使い方

同じ公開項目に rs_decimal::DecimalError を使います。インポートパスであり、別の実装やブラウザーメソッドではありません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-reexport-decimalerror" locale="ja" />

## ブラウザーの制約

SQL のテキスト／バイト入力とエラーメッセージは UTF-8 で 8192 バイトまでです。SQL テキストは外側の引用符を除去した後で[リソース検査](../resource-policy)を受けるため、引用符で指数上限を回避できません。借用呼び出しは入力を保持し、消費済みや解放済みのハンドルは再利用できません。Rust のトレイト／インポート例はネイティブテストで実行し、ブラウザーのメソッド対応を意味しません。

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
