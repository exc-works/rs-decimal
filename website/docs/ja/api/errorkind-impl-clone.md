---
locale: ja
source_revision: 0bdc18aaeece0f82160084e0821c2b46ff15a76d9ea8f28bd42767036c596312
translation_provenance: en-US-japanese-reviewed
api_id: "ErrorKind::impl Clone"
---
# ErrorKind::impl Clone

Rust の Clone 実装を使います。

```rust
impl Clone for ErrorKind
```

## 引数、戻り値、通常の使い方

Clone は値全体を保持しますが、借用された ScanValue スライスは元のデータを参照し続けます。これはネイティブ Rust のトレイト構文で、ブラウザーはメソッドとして公開しません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-impl-clone" locale="ja" />

## ブラウザーの制約

SQL のテキスト／バイト入力とエラーメッセージは UTF-8 で 8192 バイトまでです。SQL テキストは外側の引用符を除去した後で[リソース検査](../resource-policy)を受けるため、引用符で指数上限を回避できません。借用呼び出しは入力を保持し、消費済みや解放済みのハンドルは再利用できません。Rust のトレイト／インポート例はネイティブテストで実行し、ブラウザーのメソッド対応を意味しません。

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
