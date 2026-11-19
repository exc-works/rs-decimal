---
locale: ja
source_revision: 1534eba12b3e65df06a4bea76e8dd9c9cd12eeb6bcfbce92f69f83b923eef4c3
translation_provenance: en-US-japanese-reviewed
api_id: "ScanValue::Int64"
---
# ScanValue::Int64

SQL 入力の Int64 バリアントを選択します。

```rust
Int64 (i64)
```

## 引数、戻り値、通常の使い方

i64 を正確に保持します。ブラウザーは符号付き 64 ビット範囲内の bigint を要求し、JS number は拒否します。バリアントは scan に借用され、同じブラウザー実行内で再利用できます。偽造した値や別の実行の値は拒否します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-int64" locale="ja" />

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
