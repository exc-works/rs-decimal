---
locale: ja
source_revision: 8106ef71942ee657518a42cba33a247697b1ac2ebfcf011160e32c4adeb6e0b5
translation_provenance: en-US-japanese-reviewed
api_id: "reexport::ScanValue"
---
# reexport::ScanValue

クレートルートから ScanValue をインポートします。

```rust
pub use sql::ScanValue as ScanValue
```

## 引数、戻り値、通常の使い方

同じ公開項目に rs_decimal::ScanValue を使います。インポートパスであり、別の実装やブラウザーメソッドではありません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-reexport-scanvalue" locale="ja" />

## ブラウザーの制約

SQL のテキスト／バイト入力とエラーメッセージは UTF-8 で 8192 バイトまでです。SQL テキストは外側の引用符を除去した後で[リソース検査](../resource-policy)を受けるため、引用符で指数上限を回避できません。借用呼び出しは入力を保持し、消費済みや解放済みのハンドルは再利用できません。Rust のトレイト／インポート例はネイティブテストで実行し、ブラウザーのメソッド対応を意味しません。

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
