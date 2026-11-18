---
locale: ja
source_revision: d94c5a1a48ed0fbe522456d88bc4774ce805cace3b32e2a5428e2527dc3be2ce
translation_provenance: en-US-japanese-reviewed
api_id: "DecimalError::impl Error"
---
# DecimalError::impl Error

Rust の Error 実装を使います。

```rust
impl Error for DecimalError
```

## 引数、戻り値、通常の使い方

ボックス化したエラーの伝播も含め、標準 Rust Error として使います。追加の種類は連鎖する原因ではなく、source() は None です。これはネイティブ Rust のトレイト構文で、ブラウザーはメソッドとして公開しません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-impl-error" locale="ja" />

## ブラウザーの制約

SQL のテキスト／バイト入力とエラーメッセージは UTF-8 で 8192 バイトまでです。SQL テキストは外側の引用符を除去した後で[リソース検査](../resource-policy)を受けるため、引用符で指数上限を回避できません。借用呼び出しは入力を保持し、消費済みや解放済みのハンドルは再利用できません。Rust のトレイト／インポート例はネイティブテストで実行し、ブラウザーのメソッド対応を意味しません。

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
