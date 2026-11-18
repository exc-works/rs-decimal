---
locale: ja
source_revision: b5a3d9a8d6370fd602bde442c0194ed8033b601f5d907d38c4cf9f51317197c6
translation_provenance: en-US-japanese-reviewed
api_id: "ScanValue::Text"
---
# ScanValue::Text

SQL 入力の Text バリアントを選択します。

```rust
Text (& 'a str)
```

## 引数、戻り値、通常の使い方

正確な小数テキストを借用します。対応する外側の二重引用符 1 組を除去しますが、エスケープを JSON としてデコードしません。ゼロ以外の値の末尾のスケールは解析後も保持します。バリアントは scan に借用され、同じブラウザー実行内で再利用できます。偽造した値や別の実行の値は拒否します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-text" locale="ja" />

## ブラウザーの制約

SQL のテキスト／バイト入力とエラーメッセージは UTF-8 で 8192 バイトまでです。SQL テキストは外側の引用符を除去した後で[リソース検査](../resource-policy)を受けるため、引用符で指数上限を回避できません。借用呼び出しは入力を保持し、消費済みや解放済みのハンドルは再利用できません。Rust のトレイト／インポート例はネイティブテストで実行し、ブラウザーのメソッド対応を意味しません。

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
