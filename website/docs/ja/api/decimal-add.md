---
locale: ja
source_revision: 1edc6ba7e191babd98a0717864d8696548b2086ee97162b567ea1d847a357dad
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::add
---
# Decimal::add

小数を正確に加算します。

```rust
fn add (& self , other : & Self) -> Self
```

## 引数、戻り値、通常の使い方

other: &Decimal。両オペランドの大きい方のスケールで、新しい Decimal を返します。桁揃えは正確で、桁を捨てません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-add" locale="ja" />

## 関連メソッド

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
