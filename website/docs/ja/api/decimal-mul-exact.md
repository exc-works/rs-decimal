---
locale: ja
source_revision: 74fe800675809309f5a6802d09df41ce125ec348c7dc823b0a9a763856b0bb52
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::mul_exact
---
# Decimal::mul_exact

小数を正確に乗算します。

```rust
fn mul_exact (& self , other : & Self) -> Result < Self , DecimalError >
```

## 引数、戻り値、通常の使い方

other: &Decimal。結果のスケールは両オペランドのスケールの合計で、積の全桁を保持します。コアのスケールオーバーフローは Overflow を返します。丸めモードの引数はありません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-mul-exact" locale="ja" />

## 関連メソッド

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
