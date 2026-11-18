---
locale: ja
source_revision: b5e19b2f97fd7e9d8a942453bc1d3336bd82a5f0e33e943baa1dc21eb3e5ac5f
translation_provenance: en-US-japanese-reviewed
api_id: "ErrorKind::Overflow"
---
# ErrorKind::Overflow

Overflow の失敗を識別します。

```rust
Overflow
```

## 引数、戻り値、通常の使い方

係数、スケール、操作がコアの対応範囲を超えています。ブラウザーのリソースエラーは別です。ErrorKind はメッセージ文字列ではなく、安定した分類です。ブラウザーの列挙値は Rust のバリアント名です。構築したエラーでは matches() を使い、捕捉した bridge 例外では kinds 配列を使います。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-overflow" locale="ja" />

## ブラウザーの制約

SQL のテキスト／バイト入力とエラーメッセージは UTF-8 で 8192 バイトまでです。SQL テキストは外側の引用符を除去した後で[リソース検査](../resource-policy)を受けるため、引用符で指数上限を回避できません。借用呼び出しは入力を保持し、消費済みや解放済みのハンドルは再利用できません。Rust のトレイト／インポート例はネイティブテストで実行し、ブラウザーのメソッド対応を意味しません。

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
