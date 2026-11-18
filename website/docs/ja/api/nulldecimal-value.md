---
locale: ja
source_revision: 79298c474720d5ca067e6e36ecdefe721611687b58bab5fff1d35e1889208bcd
translation_provenance: en-US-japanese-reviewed
api_id: "NullDecimal::value"
---
# NullDecimal::value

標準の SQL 値を取得します。

```rust
fn value (& self) -> Option < String >
```

## 引数、戻り値、通常の使い方

引数なし。`Option<String>`/string | null を返します。無効な nullable は隠れた内部データに関係なく None/null です。有効な値は標準小数テキストを返し、内部が未初期化のゼロなら "0" です。スケールや初期化状態を変えず、小数部分の末尾のゼロを除去します。返されるテキストはスカラー値であり、引用符付き SQL クエリー断片ではありません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-nulldecimal-value" locale="ja" />

## ブラウザーの制約

SQL のテキスト／バイト入力とエラーメッセージは UTF-8 で 8192 バイトまでです。SQL テキストは外側の引用符を除去した後で[リソース検査](../resource-policy)を受けるため、引用符で指数上限を回避できません。借用呼び出しは入力を保持し、消費済みや解放済みのハンドルは再利用できません。Rust のトレイト／インポート例はネイティブテストで実行し、ブラウザーのメソッド対応を意味しません。

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
