---
locale: ja
source_revision: 36f5ff0720c14718444a86498f3f5cd1d16ba0df1d4c60c2d59357d8e469675e
translation_provenance: en-US-japanese-reviewed
api_id: "ScanValue::Int64"
---
# ScanValue::Int64

SQL 入力の Int64 バリアントを選択します。

```rust
Int64 (i64)
```

## 引数、戻り値、通常の使い方

i64 を正確に保持します。ブラウザーは符号付き 64 ビット範囲内の bigint を要求し、JS number は拒否します。バリアントは scan に借用され、同じブラウザー実行内で再利用できます。偽造した値や別の実行の値は拒否します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-int64" locale="ja" />

## ブラウザーの制約

SQL のテキスト／バイト入力とエラーメッセージは UTF-8 で 8192 バイトまでです。SQL テキストは外側の引用符を除去した後で[リソース検査](../resource-policy)を受けるため、引用符で指数上限を回避できません。借用呼び出しは入力を保持し、消費済みや解放済みのハンドルは再利用できません。Rust のトレイト／インポート例はネイティブテストで実行し、ブラウザーのメソッド対応を意味しません。

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
