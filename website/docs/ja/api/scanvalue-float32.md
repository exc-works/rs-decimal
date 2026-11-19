---
locale: ja
source_revision: 15c94f02827b09a573140cb09fd7743d1624bd74d93875d8d0450c89214f2677
translation_provenance: en-US-japanese-reviewed
api_id: "ScanValue::Float32"
---
# ScanValue::Float32

SQL 入力の Float32 バリアントを選択します。

```rust
Float32 (f32)
```

## 引数、戻り値、通常の使い方

binary32 を保持します。ブラウザーは number を受け取り、Rust は scan 前に f32 に狭めます。NaN、無限大、変換時のオーバーフローは Unmarshal です。バリアントは scan に借用され、同じブラウザー実行内で再利用できます。偽造した値や別の実行の値は拒否します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-float32" locale="ja" />

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
