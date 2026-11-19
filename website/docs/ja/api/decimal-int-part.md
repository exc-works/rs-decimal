---
locale: ja
source_revision: 0b7e4ede82a7865b24f3ea40debe791505bf9ac255c8e2f8790aa1271d0fdf3a
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::int_part
---
# Decimal::int_part

ゼロ方向に切り捨てて整数部分を取り出します。

```rust
fn int_part (& self) -> BigInt
```

## 引数、戻り値、通常の使い方

引数なし。Rust は BigInt、JavaScript は bigint を返し、浮動小数点変換は行いません。-12.75 の結果は -12 であり、floor(-12.75) = -13 ではありません。既定のゼロはゼロを返します。例では JSON でも任意精度を保つため整数を文字列として出力します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-int-part" locale="ja" />

## 関連メソッド

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
