---
locale: ja
source_revision: 174ec0afec71ca2dec4f0c9c844589a1e01782200d059f787f800837923cca1b
translation_provenance: en-US-japanese-reviewed
api_id: "ScanValue::Unsupported"
---
# ScanValue::Unsupported

SQL 入力の Unsupported バリアントを選択します。

```rust
Unsupported
```

## 引数、戻り値、通常の使い方

未対応のドライバー入力を型変換せずに表します。scan は Unmarshal を返し、既存の値を保持します。バリアントは scan に借用され、同じブラウザー実行内で再利用できます。偽造した値や別の実行の値は拒否します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-unsupported" locale="ja" />

## ブラウザーの制約

SQL のテキスト／バイト入力とエラーメッセージは UTF-8 で 8192 バイトまでです。SQL テキストは外側の引用符を除去した後で[リソース検査](../resource-policy)を受けるため、引用符で指数上限を回避できません。借用呼び出しは入力を保持し、消費済みや解放済みのハンドルは再利用できません。Rust のトレイト／インポート例はネイティブテストで実行し、ブラウザーのメソッド対応を意味しません。

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
