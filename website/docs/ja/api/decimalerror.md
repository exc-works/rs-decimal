---
locale: ja
source_revision: e7411932b8884630967e9c3e34bf9db436dc8920df8139ae778ac83fc80fdcb3
translation_provenance: en-US-japanese-reviewed
api_id: "DecimalError"
---
# DecimalError

メッセージと 1 つ以上のライブラリエラー種別を保持します。

```rust
pub struct DecimalError
```

## 引数、戻り値、通常の使い方

DecimalError は人が読めるメッセージと、重複を除去して順序付けた種類を保存します。with_kind は値を消費し、matches と kinds は借用します。Rust の Error と Display を実装します。ブラウザーの名前付きコンストラクターは、message、matches()、kinds()、with_kind() を持つ解放可能な Rust ベースの値を作成します。送出されたライブラリ例外は category="library" と kinds 配列を持つネイティブ JS Error です。そこでは e.kinds.includes(ErrorKind.Unmarshal) を使います。parameter/resource/moved の bridge エラーは別で、ライブラリの種類は持ちません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror" locale="ja" />

## ブラウザーの制約

SQL のテキスト／バイト入力とエラーメッセージは UTF-8 で 8192 バイトまでです。SQL テキストは外側の引用符を除去した後で[リソース検査](../resource-policy)を受けるため、引用符で指数上限を回避できません。借用呼び出しは入力を保持し、消費済みや解放済みのハンドルは再利用できません。Rust のトレイト／インポート例はネイティブテストで実行し、ブラウザーのメソッド対応を意味しません。

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
