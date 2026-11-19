---
locale: ja
source_revision: cd65ee6368cc890889ec3fceb39f3b12814d9f2d0fe811e6c57c730bc5d81b5b
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

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
