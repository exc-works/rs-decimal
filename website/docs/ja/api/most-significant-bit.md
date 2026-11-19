---
locale: ja
source_revision: e0008f3df5627ec04d695568de0e59f79f0a5617252c018a2798ad7b622bab4d
translation_provenance: en-US-japanese-reviewed
api_id: most_significant_bit
---
# most_significant_bit

最上位の 1 ビットの位置を 0 始まりで求めます。

```rust
fn most_significant_bit (value : & BigInt) -> Result < usize , DecimalError >
```

## 引数、戻り値、通常の使い方

value: &BigInt を bigint で表します。Result<usize, DecimalError>/number を返します。0 と 1 はともに 0、120 は 6 です。負の入力は InvalidArgument です。これは自由関数で、Decimal メソッドではありません。対象が係数なら coefficient() を渡します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-most-significant-bit" locale="ja" />

## 関連メソッド

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
