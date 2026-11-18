---
locale: ja
source_revision: c70b9c986ae1c2d1db6f94784745de5a4d3bb8a7cf3f6af3550e9be48e0f551f
translation_provenance: en-US-japanese-reviewed
api_id: "DecimalError::new"
---
# DecimalError::new

1 つの種類を持つエラーを作成します。

```rust
fn new (kind : ErrorKind , message : impl Into < String >) -> Self
```

## 引数、戻り値、通常の使い方

kind: ErrorKind、message: impl `Into<String>`/string。新しい DecimalError を返します。メッセージは人向けであり、種類への所属を決めません。ブラウザーの名前付きコンストラクターは、message、matches()、kinds()、with_kind() を持つ解放可能な Rust ベースの値を作成します。送出されたライブラリ例外は category="library" と kinds 配列を持つネイティブ JS Error です。そこでは e.kinds.includes(ErrorKind.Unmarshal) を使います。parameter/resource/moved の bridge エラーは別で、ライブラリの種類は持ちません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-new" locale="ja" />

## ブラウザーの制約

SQL のテキスト／バイト入力とエラーメッセージは UTF-8 で 8192 バイトまでです。SQL テキストは外側の引用符を除去した後で[リソース検査](../resource-policy)を受けるため、引用符で指数上限を回避できません。借用呼び出しは入力を保持し、消費済みや解放済みのハンドルは再利用できません。Rust のトレイト／インポート例はネイティブテストで実行し、ブラウザーのメソッド対応を意味しません。

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
