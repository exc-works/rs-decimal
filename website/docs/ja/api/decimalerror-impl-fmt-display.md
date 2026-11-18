---
locale: ja
source_revision: 313aa913e617945bd29e0e39f2ee0488ae3c0d32ae43ffdacbbf7badbfb3dbb1
translation_provenance: en-US-japanese-reviewed
api_id: "DecimalError::impl fmt :: Display"
---
# DecimalError::impl fmt :: Display

Rust の fmt 実装を使います。

```rust
impl fmt :: Display for DecimalError
```

## 引数、戻り値、通常の使い方

Display はメッセージだけを書き込みます。format! または to_string() を使い、機械処理では kinds を別に調べてください。これはネイティブ Rust のトレイト構文で、ブラウザーはメソッドとして公開しません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-impl-fmt-display" locale="ja" />

## ブラウザーの制約

SQL のテキスト／バイト入力とエラーメッセージは UTF-8 で 8192 バイトまでです。SQL テキストは外側の引用符を除去した後で[リソース検査](../resource-policy)を受けるため、引用符で指数上限を回避できません。借用呼び出しは入力を保持し、消費済みや解放済みのハンドルは再利用できません。Rust のトレイト／インポート例はネイティブテストで実行し、ブラウザーのメソッド対応を意味しません。

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
