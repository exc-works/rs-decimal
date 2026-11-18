---
locale: ja
source_revision: 4a909248a2e14bd655a50dd72e826f6d87a3347b735e76ddfe31ae9a4d4b4671
translation_provenance: en-US-japanese-reviewed
api_id: "ScanValue::Bytes"
---
# ScanValue::Bytes

SQL 入力の Bytes バリアントを選択します。

```rust
Bytes (& 'a [u8])
```

## 引数、戻り値、通常の使い方

バイナリ／BSON の通信データではなく、小数テキストのバイト列を借用します。ブラウザーは scan まで Uint8Array を保持し、それ以前の編集を反映します。scan は入力を変更しません。不正な UTF-8 は小数解析に失敗します。バリアントは scan に借用され、同じブラウザー実行内で再利用できます。偽造した値や別の実行の値は拒否します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-bytes" locale="ja" />

## ブラウザーの制約

SQL のテキスト／バイト入力とエラーメッセージは UTF-8 で 8192 バイトまでです。SQL テキストは外側の引用符を除去した後で[リソース検査](../resource-policy)を受けるため、引用符で指数上限を回避できません。借用呼び出しは入力を保持し、消費済みや解放済みのハンドルは再利用できません。Rust のトレイト／インポート例はネイティブテストで実行し、ブラウザーのメソッド対応を意味しません。

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
