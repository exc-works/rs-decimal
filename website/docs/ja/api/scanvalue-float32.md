---
locale: ja
source_revision: dbb045138dac94f7b2691e8576e6af500d5ead689869256e178b5069283ab80d
translation_provenance: en-US-japanese-reviewed
api_id: "ScanValue::Float32"
---
# ScanValue::Float32

SQL 入力の Float32 バリアントを選択します。

```rust
Float32 (f32)
```

## 引数、戻り値、通常の使い方

binary32 を保持します。ブラウザーは number を受け取り、Rust は scan 前に f32 に狭めます。NaN、無限大、変換時のオーバーフローは Unmarshal です。バリアントは scan に借用され、同じブラウザー実行内で再利用できます。偽造した値や別の実行の値は拒否します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-float32" locale="ja" />

## ブラウザーの制約

SQL のテキスト／バイト入力とエラーメッセージは UTF-8 で 8192 バイトまでです。SQL テキストは外側の引用符を除去した後で[リソース検査](../resource-policy)を受けるため、引用符で指数上限を回避できません。借用呼び出しは入力を保持し、消費済みや解放済みのハンドルは再利用できません。Rust のトレイト／インポート例はネイティブテストで実行し、ブラウザーのメソッド対応を意味しません。

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
