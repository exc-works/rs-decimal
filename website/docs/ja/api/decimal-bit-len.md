---
locale: ja
source_revision: cd78c2dcf4b67b04ef802fd73ba3ef0da02aad24d1bb17be5cc5df5d221df3cd
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::bit_len
---
# Decimal::bit_len

スケール適用前の係数の絶対値のビット数を数えます。

```rust
fn bit_len (& self) -> usize
```

## 引数、戻り値、通常の使い方

引数なし。Rust は usize、JavaScript は number を返します。スケールは適用せず、符号ビットは数えません。-1.20 の係数は -120 で、ビット長は 7 です。既定のゼロを含むゼロには 0 を返します。最上位の 1 ビットの位置ではありません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-bit-len" locale="ja" />

## 関連メソッド

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
