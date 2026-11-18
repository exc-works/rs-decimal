---
locale: ja
source_revision: c136bea39fff64039ef208c269a0655f2c126075df944740bc4d24f9d953ef55
translation_provenance: en-US-japanese-reviewed
api_id: "ErrorKind"
---
# ErrorKind

安定したライブラリエラーの種類を説明します。

```rust
pub enum ErrorKind
```

## 引数、戻り値、通常の使い方

列挙型には 11 種類あり、Rust でコピーと順序比較ができます。ブラウザーの ErrorKind は Rust から読み込んだ凍結済みの名前マップです。個別のエラー処理にはメッセージ比較ではなく、種類への所属を使います。ブラウザーの名前付きコンストラクターは、message、matches()、kinds()、with_kind() を持つ解放可能な Rust ベースの値を作成します。送出されたライブラリ例外は category="library" と kinds 配列を持つネイティブ JS Error です。そこでは e.kinds.includes(ErrorKind.Unmarshal) を使います。parameter/resource/moved の bridge エラーは別で、ライブラリの種類は持ちません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind" locale="ja" />

## ブラウザーの制約

SQL のテキスト／バイト入力とエラーメッセージは UTF-8 で 8192 バイトまでです。SQL テキストは外側の引用符を除去した後で[リソース検査](../resource-policy)を受けるため、引用符で指数上限を回避できません。借用呼び出しは入力を保持し、消費済みや解放済みのハンドルは再利用できません。Rust のトレイト／インポート例はネイティブテストで実行し、ブラウザーのメソッド対応を意味しません。

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
