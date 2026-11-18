---
locale: ja
source_revision: 292844a00b6e84cbd25e16d6e84fa03c5565c434cf287f28e2959e5ca0c46780
translation_provenance: en-US-japanese-reviewed
api_id: "DecimalError::matches"
---
# DecimalError::matches

指定した種類を持つか調べます。

```rust
fn matches (& self , kind : ErrorKind) -> bool
```

## 引数、戻り値、通常の使い方

kind: ErrorKind。bool/boolean を返します。借用はエラーを変更しません。1 つのエラーは同時に複数の種類に一致できます。ブラウザーの名前付きコンストラクターは、message、matches()、kinds()、with_kind() を持つ解放可能な Rust ベースの値を作成します。送出されたライブラリ例外は category="library" と kinds 配列を持つネイティブ JS Error です。そこでは e.kinds.includes(ErrorKind.Unmarshal) を使います。parameter/resource/moved の bridge エラーは別で、ライブラリの種類は持ちません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-matches" locale="ja" />

## ブラウザーの制約

SQL のテキスト／バイト入力とエラーメッセージは UTF-8 で 8192 バイトまでです。SQL テキストは外側の引用符を除去した後で[リソース検査](../resource-policy)を受けるため、引用符で指数上限を回避できません。借用呼び出しは入力を保持し、消費済みや解放済みのハンドルは再利用できません。Rust のトレイト／インポート例はネイティブテストで実行し、ブラウザーのメソッド対応を意味しません。

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
