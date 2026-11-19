---
locale: ja
source_revision: 78277d1082dac8a98f79b7018413113c09afb0c224a6c82ae6c179432b0517f1
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

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
