---
locale: ja
source_revision: 51d80d897f775a7466e06561586b9d500a34859b5d00dfee1f00a368a7a33f1e
translation_provenance: en-US-japanese-reviewed
api_id: "DecimalError::kinds"
---
# DecimalError::kinds

エラーの全種類を読み取ります。

```rust
fn kinds (& self) -> & [ErrorKind]
```

## 引数、戻り値、通常の使い方

引数なし。&[ErrorKind]/独立したバリアント名の配列を返します。種類は重複を除去し、アルファベット順ではなく Rust の列挙順で並びます。ブラウザーの名前付きコンストラクターは、message、matches()、kinds()、with_kind() を持つ解放可能な Rust ベースの値を作成します。送出されたライブラリ例外は category="library" と kinds 配列を持つネイティブ JS Error です。そこでは e.kinds.includes(ErrorKind.Unmarshal) を使います。parameter/resource/moved の bridge エラーは別で、ライブラリの種類は持ちません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-kinds" locale="ja" />

## ブラウザーの制約

SQL のテキスト／バイト入力とエラーメッセージは UTF-8 で 8192 バイトまでです。SQL テキストは外側の引用符を除去した後で[リソース検査](../resource-policy)を受けるため、引用符で指数上限を回避できません。借用呼び出しは入力を保持し、消費済みや解放済みのハンドルは再利用できません。Rust のトレイト／インポート例はネイティブテストで実行し、ブラウザーのメソッド対応を意味しません。

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
