---
locale: ja
source_revision: 5aa2d7144a94e759cd81dc307aff2c18ce7db36a83dc3e91c6d08804e1cdf2c5
translation_provenance: en-US-japanese-reviewed
api_id: "DecimalError::impl PartialEq"
---
# DecimalError::impl PartialEq

Rust の PartialEq 実装を使います。

```rust
impl PartialEq for DecimalError
```

## 引数、戻り値、通常の使い方

等価性はエラーメッセージと種類全体、または正確な列挙子を比較します。DecimalError の等価性は 1 種類への一致より厳密です。これはネイティブ Rust のトレイト構文で、ブラウザーはメソッドとして公開しません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-impl-partialeq" locale="ja" />

## ブラウザーの制約

SQL のテキスト／バイト入力とエラーメッセージは UTF-8 で 8192 バイトまでです。SQL テキストは外側の引用符を除去した後で[リソース検査](../resource-policy)を受けるため、引用符で指数上限を回避できません。借用呼び出しは入力を保持し、消費済みや解放済みのハンドルは再利用できません。Rust のトレイト／インポート例はネイティブテストで実行し、ブラウザーのメソッド対応を意味しません。

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
