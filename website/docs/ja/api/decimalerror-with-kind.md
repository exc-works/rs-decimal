---
locale: ja
source_revision: f5327cbdbb40212ac4046096b90742ab088011d23930faef1e12c326d8e03e09
translation_provenance: en-US-japanese-reviewed
api_id: "DecimalError::with_kind"
---
# DecimalError::with_kind

エラーを消費して種類を追加します。

```rust
fn with_kind (mut self , kind : ErrorKind) -> Self
```

## 引数、戻り値、通常の使い方

kind: ErrorKind。self を消費し、新しい DecimalError を返します。既存の種類とメッセージを保持し、重複は無視します。元のブラウザーハンドルは使えなくなります。不正な JS kind 値はムーブ前に拒否します。ブラウザーの名前付きコンストラクターは、message、matches()、kinds()、with_kind() を持つ解放可能な Rust ベースの値を作成します。送出されたライブラリ例外は category="library" と kinds 配列を持つネイティブ JS Error です。そこでは e.kinds.includes(ErrorKind.Unmarshal) を使います。parameter/resource/moved の bridge エラーは別で、ライブラリの種類は持ちません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-with-kind" locale="ja" />

## ブラウザーの制約

SQL のテキスト／バイト入力とエラーメッセージは UTF-8 で 8192 バイトまでです。SQL テキストは外側の引用符を除去した後で[リソース検査](../resource-policy)を受けるため、引用符で指数上限を回避できません。借用呼び出しは入力を保持し、消費済みや解放済みのハンドルは再利用できません。Rust のトレイト／インポート例はネイティブテストで実行し、ブラウザーのメソッド対応を意味しません。

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
