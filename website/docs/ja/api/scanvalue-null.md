---
locale: ja
source_revision: 17f954a2ac80c90ce4a20dd8e36b0981b73e6748171ba829abe55cdfdc69b2b3
translation_provenance: en-US-japanese-reviewed
api_id: "ScanValue::Null"
---
# ScanValue::Null

SQL 入力の Null バリアントを選択します。

```rust
Null
```

## 引数、戻り値、通常の使い方

SQL NULL を表します。scan は Decimal を未初期化のゼロにリセットし、NullDecimal をクリアします。バリアントは scan に借用され、同じブラウザー実行内で再利用できます。偽造した値や別の実行の値は拒否します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-null" locale="ja" />

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
