---
locale: ja
source_revision: 6c951b2ad37813f4b84e7dc136137d89576bf098308ff5b7d85a1bf0a3c58e09
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::new_from_float64
---
# Decimal::new_from_float64

浮動小数点へ戻すと同じ値になる、最短の十進表現から構築します。

```rust
fn new_from_float64 (value : f64) -> Result < Self , DecimalError >
```

## 引数、戻り値、通常の使い方

value: f64 を JS number で渡します。二進浮動小数点ですでに失われた小数の桁は復元できません。正確な小数入力には new_from_string を使ってください。NaN と正負の無限大は InvalidFormat です。負のゼロは符号のない初期化済み Decimal ゼロになります。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-new-from-float64" locale="ja" />

## 関連メソッド

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
