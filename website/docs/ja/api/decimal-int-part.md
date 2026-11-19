---
locale: ja
source_revision: b7ff1544694c51babb7af7cb99735ecc6687d86e9cd9a108af321bed93252beb
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::int_part
---
# Decimal::int_part

ゼロ方向に切り捨てて整数部分を取り出します。

```rust
fn int_part (& self) -> BigInt
```

## 引数、戻り値、通常の使い方

引数なし。Rust は BigInt、bridge は bigint を返し、浮動小数点変換は行いません。-12.75 の結果は -12 であり、floor(-12.75) = -13 ではありません。既定のゼロはゼロを返します。例では JSON でも任意精度を保つため整数を文字列として出力します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-int-part" locale="ja" />

## 関連メソッド

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
