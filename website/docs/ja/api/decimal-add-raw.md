---
locale: ja
source_revision: ad5bc09e58ba54af3a20899560f38f4b2682294a4c7489043e5331092c1467a0
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::add_raw
---
# Decimal::add_raw

スケール適用前の係数を i64 整数分だけ変更します。

```rust
fn add_raw (& self , value : i64) -> Self
```

## 引数、戻り値、通常の使い方

value: i64。JavaScript では bigint で表します。スケールは保持するため、スケール 2 で 1 を加減すると数値は 1 ではなく 0.01 変わります。Rust は新しい Decimal を返します。number と i64 範囲外の bigint は JavaScript が拒否します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-add-raw" locale="ja" />

## 関連メソッド

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
