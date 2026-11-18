---
locale: ja
source_revision: 93c9a590df79625539c288c9f6a42d27aa1dba571b28363b98f9ec4642104e16
translation_provenance: en-US-japanese-reviewed
api_id: "ErrorKind::impl Ord"
---
# ErrorKind::impl Ord

Rust の Ord 実装を使います。

```rust
impl Ord for ErrorKind
```

## 引数、戻り値、通常の使い方

ErrorKind を重大度や名前のアルファベット順ではなく、Rust の宣言順に並べます。これはネイティブ Rust のトレイト構文で、ブラウザーはメソッドとして公開しません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-impl-ord" locale="ja" />

## ブラウザーの制約

SQL のテキスト／バイト入力とエラーメッセージは UTF-8 で 8192 バイトまでです。SQL テキストは外側の引用符を除去した後で[リソース検査](../resource-policy)を受けるため、引用符で指数上限を回避できません。借用呼び出しは入力を保持し、消費済みや解放済みのハンドルは再利用できません。Rust のトレイト／インポート例はネイティブテストで実行し、ブラウザーのメソッド対応を意味しません。

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
