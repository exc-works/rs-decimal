---
locale: ja
source_revision: 8e304b6c42ea7cec512b7054ed8c7c7654d4540ebdd19dbc647e83dc885c5809
translation_provenance: en-US-japanese-reviewed
api_id: "ScanValue::Null"
---
# ScanValue::Null

SQL 入力の Null バリアントを選択します。

```rust
Null
```

## 引数、戻り値、通常の使い方

SQL NULL を表します。scan は Decimal を未初期化のゼロにリセットし、NullDecimal をクリアします。バリアントは scan に借用され、同じブラウザー実行内で再利用できます。偽造した値や別の実行の値は拒否します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-null" locale="ja" />

## ブラウザーの制約

SQL のテキスト／バイト入力とエラーメッセージは UTF-8 で 8192 バイトまでです。SQL テキストは外側の引用符を除去した後で[リソース検査](../resource-policy)を受けるため、引用符で指数上限を回避できません。借用呼び出しは入力を保持し、消費済みや解放済みのハンドルは再利用できません。Rust のトレイト／インポート例はネイティブテストで実行し、ブラウザーのメソッド対応を意味しません。

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
