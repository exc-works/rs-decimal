---
locale: ja
source_revision: e5103a64d713be93c5c51f691b143dc290327bd825e0acd2029d936b2165437e
translation_provenance: en-US-japanese-reviewed
api_id: "ScanValue::Float64"
---
# ScanValue::Float64

SQL 入力の Float64 バリアントを選択します。

```rust
Float64 (f64)
```

## 引数、戻り値、通常の使い方

binary64 を保持します。有限値はコアの最短小数変換を使い、NaN と無限大は Unmarshal です。丸め済みの JS number から元の桁は復元できません。バリアントは scan に借用され、同じブラウザー実行内で再利用できます。偽造した値や別の実行の値は拒否します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-float64" locale="ja" />

## ブラウザーの制約

SQL のテキスト／バイト入力とエラーメッセージは UTF-8 で 8192 バイトまでです。SQL テキストは外側の引用符を除去した後で[リソース検査](../resource-policy)を受けるため、引用符で指数上限を回避できません。借用呼び出しは入力を保持し、消費済みや解放済みのハンドルは再利用できません。Rust のトレイト／インポート例はネイティブテストで実行し、ブラウザーのメソッド対応を意味しません。

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
