---
locale: ja
source_revision: 84ba3002f85664068a9d9ececc408a7d6f0214a486be55b19a41d101ddf845eb
translation_provenance: en-US-japanese-reviewed
api_id: RoundingMode::HalfEven
---
# RoundingMode::HalfEven

最も近い値へ丸め、中間では残る桁が偶数になる方を選びます。

```rust
HalfEven = 5
```

## 引数、戻り値、通常の使い方

整数の判別値は 5 です。マジックナンバーではなく名前付きの値を使ってください。規則は正負の両方に適用されます。Down と Up はゼロを基準にした方向であり、負または正の無限大ではありません。Half 系はちょうど中間の場合だけ異なります。Floor という列挙子はないため、その方向には floor_with_prec を使います。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-halfeven" locale="ja" />

## 関連メソッド

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
