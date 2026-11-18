---
locale: ja
source_revision: 7a9119eb74e99c5e35f09e87df2556b438ba2f0ad510d5e668cdcc13dc40e550
translation_provenance: en-US-japanese-reviewed
api_id: "ErrorKind::NoConvergence"
---
# ErrorKind::NoConvergence

NoConvergence の失敗を識別します。

```rust
NoConvergence
```

## 引数、戻り値、通常の使い方

上限付きのアルゴリズムが丸め結果を証明できませんでした。数学的な結果が存在しないという意味ではありません。ErrorKind はメッセージ文字列ではなく、安定した分類です。ブラウザーの列挙値は Rust のバリアント名です。構築したエラーでは matches() を使い、捕捉した bridge 例外では kinds 配列を使います。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-noconvergence" locale="ja" />

## ブラウザーの制約

SQL のテキスト／バイト入力とエラーメッセージは UTF-8 で 8192 バイトまでです。SQL テキストは外側の引用符を除去した後で[リソース検査](../resource-policy)を受けるため、引用符で指数上限を回避できません。借用呼び出しは入力を保持し、消費済みや解放済みのハンドルは再利用できません。Rust のトレイト／インポート例はネイティブテストで実行し、ブラウザーのメソッド対応を意味しません。

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
