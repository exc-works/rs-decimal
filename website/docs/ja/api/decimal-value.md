---
locale: ja
source_revision: ab8ea6bf03391270a33235fc649739ef25b1cdab822e201005d40879c5be9299
translation_provenance: en-US-japanese-reviewed
api_id: "Decimal::value"
---
# Decimal::value

標準の SQL 値を取得します。

```rust
fn value (& self) -> String
```

## 引数、戻り値、通常の使い方

引数なし。String/string を返します。未初期化の Decimal でも NULL ではない "0" を返します。SQL NULL が必要なら NullDecimal を使います。スケールや初期化状態を変えず、小数部分の末尾のゼロを除去します。返されるテキストはスカラー値であり、引用符付き SQL クエリー断片ではありません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimal-value" locale="ja" />

## ブラウザーの制約

SQL のテキスト／バイト入力とエラーメッセージは UTF-8 で 8192 バイトまでです。SQL テキストは外側の引用符を除去した後で[リソース検査](../resource-policy)を受けるため、引用符で指数上限を回避できません。借用呼び出しは入力を保持し、消費済みや解放済みのハンドルは再利用できません。Rust のトレイト／インポート例はネイティブテストで実行し、ブラウザーのメソッド対応を意味しません。

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
