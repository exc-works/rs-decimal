---
locale: ja
source_revision: 50372ca1028f4ab77e1e376da48c589b0fdad5a066b7c436d3b5780901b17b77
translation_provenance: en-US-japanese-reviewed
api_id: RoundingMode::Unnecessary
---
# RoundingMode::Unnecessary

正確な結果のみを許可します。

```rust
Unnecessary = 6
```

## 引数、戻り値、通常の使い方

整数の判別値は 6 です。マジックナンバーではなく名前付きの値を使ってください。規則は正負の両方に適用されます。ゼロの桁を捨てても正確ですが、ゼロ以外の桁を 1 つでも捨てると RoundUnnecessary を返します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-unnecessary" locale="ja" />

## 関連メソッド

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
