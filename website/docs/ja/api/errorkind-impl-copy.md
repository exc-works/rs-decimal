---
locale: ja
source_revision: a66891e5a69e79ad1f199a0dd34da946899f3deff1662608f76ae3c569145632
translation_provenance: en-US-japanese-reviewed
api_id: "ErrorKind::impl Copy"
---
# ErrorKind::impl Copy

Rust の Copy 実装を使います。

```rust
impl Copy for ErrorKind
```

## 引数、戻り値、通常の使い方

Copy はこの小さな列挙型を複製します。借用されたスカラーデータは深くコピーしません。これはネイティブ Rust のトレイト構文で、ブラウザーはメソッドとして公開しません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-impl-copy" locale="ja" />

## ブラウザーの制約

SQL のテキスト／バイト入力とエラーメッセージは UTF-8 で 8192 バイトまでです。SQL テキストは外側の引用符を除去した後で[リソース検査](../resource-policy)を受けるため、引用符で指数上限を回避できません。借用呼び出しは入力を保持し、消費済みや解放済みのハンドルは再利用できません。Rust のトレイト／インポート例はネイティブテストで実行し、ブラウザーのメソッド対応を意味しません。

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
