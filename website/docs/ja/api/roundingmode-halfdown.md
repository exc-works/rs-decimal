---
locale: ja
source_revision: 80ca29e7b285c6bbe14fbaae400143c560d7d0375268641d09bff74efd458b6d
translation_provenance: en-US-japanese-reviewed
api_id: RoundingMode::HalfDown
---
# RoundingMode::HalfDown

最も近い値へ丸め、中間ではゼロ方向を選びます。

```rust
HalfDown = 4
```

## 引数、戻り値、通常の使い方

整数の判別値は 4 です。マジックナンバーではなく名前付きの値を使ってください。規則は正負の両方に適用されます。Down と Up はゼロを基準にした方向であり、負または正の無限大ではありません。Half 系はちょうど中間の場合だけ異なります。Floor という列挙子はないため、その方向には floor_with_prec を使います。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-halfdown" locale="ja" />

## 関連メソッド

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
