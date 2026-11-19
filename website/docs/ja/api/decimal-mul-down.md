---
locale: ja
source_revision: 8f612ca2b723227e5991f08e31b92d45cb4ea058472e86db2c02e8a1c4cc372e
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::mul_down
---
# Decimal::mul_down

指定した出力精度で小数を乗算します。

```rust
fn mul_down (& self , other : & Self) -> Self
```

## 引数、戻り値、通常の使い方

other: &Decimal。結果のスケールは両オペランドの大きい方であり、合計ではありません。積の全桁を保持するには mul_exact を使います。Down は負の結果もゼロ方向に切り捨てます。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-mul-down" locale="ja" />

## 関連メソッド

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
