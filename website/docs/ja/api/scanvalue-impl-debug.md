---
locale: ja
source_revision: 9436e2c2329251e2adecf17884000bc9408c12b9f0b7fd847a1b861820048eb7
translation_provenance: en-US-japanese-reviewed
api_id: "ScanValue::impl Debug"
---
# ScanValue::impl Debug

Rust の Debug 実装を使います。

```rust
impl Debug for ScanValue
```

## 引数、戻り値、通常の使い方

Debug は診断用テキストを提供し、安定した通信シリアライズ形式ではありません。これはネイティブ Rust のトレイト構文で、ブラウザーはメソッドとして公開しません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-impl-debug" locale="ja" />

## ブラウザーの制約

SQL のテキスト／バイト入力とエラーメッセージは UTF-8 で 8192 バイトまでです。SQL テキストは外側の引用符を除去した後で[リソース検査](../resource-policy)を受けるため、引用符で指数上限を回避できません。借用呼び出しは入力を保持し、消費済みや解放済みのハンドルは再利用できません。Rust のトレイト／インポート例はネイティブテストで実行し、ブラウザーのメソッド対応を意味しません。

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
