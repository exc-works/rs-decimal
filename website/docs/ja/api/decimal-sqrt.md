---
locale: ja
source_revision: 0bbd6c78da40405c6e9ad9c68a14878f98d98b4601eb8ff1296abba0bbf809ae
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::sqrt
---
# Decimal::sqrt

平方根を計算します。Rust は Result<Decimal, DecimalError>、bridge は Decimal または構造化された例外を返します。次数の引数はありません。負の入力は NegativeRoot を返します。既定精度は max(入力の小数点以下の桁数, 20) であり、有効数字 20 桁ではありません。丸めは最近接偶数丸めです。精度 0 の sqrt(6.25) のように正確に中間になる根は 2 に丸めます。approx_root という名前でも、Rust は整数比較で丸めを証明します。コアの計算量や反復上限に達すると NoConvergence を返します。入力は借用して変更せず、既定のゼロを含む結果は初期化済みです。

```rust
fn sqrt (& self) -> Result < Self , DecimalError >
```

## 通常の使い方

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="roots-decimal-sqrt" locale="ja" />

## 関連メソッド

[sqrt](./decimal-sqrt) · [approx_root_with_prec](./decimal-approx-root-with-prec) · [power](./decimal-power)
