---
locale: ja
source_revision: 8064cdb13ca114f6f1b29d727ed53f33f638f79926b35e606248ced643f822e9
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

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
