---
locale: ja
source_revision: 3e67428ba1ea89053920f2f5796e826dd297f81c12e673f6f4e20f40216c28ff
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::big_rat
---
# Decimal::big_rat

約分した正確な有理数値を読み取ります。

```rust
fn big_rat (& self) -> BigRational
```

## 引数、戻り値、通常の使い方

引数なし。Rust は BigRational を返します。bridge は分母を正に正規化した、不変の {numerator: bigint, denominator: bigint} スナップショットを返します。小数のスケールや末尾のゼロは有理数のメタデータとして保持されず、ゼロは 0/1 になります。入力は借用し、変更しません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-big-rat" locale="ja" />

## 関連メソッド

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
