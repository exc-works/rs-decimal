---
locale: ja
source_revision: b276bcddff5985f20e7905654e4b3cef6f352b3cfa139550a92b9425bd777e62
translation_provenance: en-US-japanese-reviewed
api_id: "ScanValue::Float64"
---
# ScanValue::Float64

SQL 入力の Float64 バリアントを選択します。

```rust
Float64 (f64)
```

## 引数、戻り値、通常の使い方

binary64 を保持します。有限値はコアの最短小数変換を使い、NaN と無限大は Unmarshal です。丸め済みの JS number から元の桁は復元できません。バリアントは scan に借用され、同じブラウザー実行内で再利用できます。偽造した値や別の実行の値は拒否します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-float64" locale="ja" />

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
