---
locale: ja
source_revision: 584886d027ae1ffb7977be9193069c421cfe24c70d485c0551ed585a1d8e7407
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::strip_trailing_zeros
---
# Decimal::strip_trailing_zeros

数値を変えず、小数部分の不要な末尾のゼロを除去します。

```rust
fn strip_trailing_zeros (& self) -> Self
```

## 引数、戻り値、通常の使い方

引数なし。独立した Decimal を返します。スケールが正の間だけ 10 の因数を除去するため、1200 のような整数のゼロは失われません。数値ゼロはすべてスケール 0 の初期化済みゼロになります。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-strip-trailing-zeros" locale="ja" />

## 関連メソッド

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
