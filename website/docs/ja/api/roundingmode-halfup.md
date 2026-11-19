---
locale: ja
source_revision: bb4dd0948b4d4be09c12115b5c13abd8421926284cdbcfed281913fc4636aeb5
translation_provenance: en-US-japanese-reviewed
api_id: RoundingMode::HalfUp
---
# RoundingMode::HalfUp

最も近い値へ丸め、中間ではゼロから遠ざかる方向を選びます。

```rust
HalfUp = 3
```

## 引数、戻り値、通常の使い方

整数の判別値は 3 です。マジックナンバーではなく名前付きの値を使ってください。規則は正負の両方に適用されます。Down と Up はゼロを基準にした方向であり、負または正の無限大ではありません。Half 系はちょうど中間の場合だけ異なります。Floor という列挙子はないため、その方向には floor_with_prec を使います。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-halfup" locale="ja" />

## 関連メソッド

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
