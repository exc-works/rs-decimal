---
locale: ja
source_revision: 1364e2c70db38e747614af95f661a59bb822316d59013022933d2987f410c846
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::sqrt_with_prec
---
# Decimal::sqrt_with_prec

平方根を計算します。Rust は Result<Decimal, DecimalError>、bridge は Decimal または構造化された例外を返します。次数の引数はありません。負の入力は NegativeRoot を返します。prec: isize を整数 number で渡し、小数点以下の桁数を選びます。負の精度は定義域の検査前に InvalidPrecision を返します。丸めは最近接偶数丸めです。精度 0 の sqrt(6.25) のように正確に中間になる根は 2 に丸めます。approx_root という名前でも、Rust は整数比較で丸めを証明します。コアの計算量や反復上限に達すると NoConvergence を返します。入力は借用して変更せず、既定のゼロを含む結果は初期化済みです。

```rust
fn sqrt_with_prec (& self , prec : isize) -> Result < Self , DecimalError >
```

## 通常の使い方

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="roots-decimal-sqrt-with-prec" locale="ja" />

## 関連メソッド

[sqrt](./decimal-sqrt) · [approx_root_with_prec](./decimal-approx-root-with-prec) · [power](./decimal-power)
