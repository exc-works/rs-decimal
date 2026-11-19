---
locale: ja
source_revision: 07fd0624795c1d5a39d29225db8108c9026886d41981fb4ff89e2fb3be01274d
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::exp_with_prec
---
# Decimal::exp_with_prec

指数関数 e^xを計算します。Rust は Result<Decimal, DecimalError>、bridge は Decimal または構造化された例外を返します。prec: 非負の isize を整数 number で表し、小数点以下の桁数を選びます。負の精度は定義域の検査前に InvalidPrecision を返します。コアではすべての実数入力を許可します。exp(0) は初期化済みの 1 です。十分に負の引数は、要求精度で初期化済みゼロへ正しく丸められる場合があります。コアは値を囲む区間を使い、丸めを証明できたときだけ結果を受理します。mode 引数のない最近接偶数丸めです。コアの上限内で証明できない場合は、未検証の近似値ではなく NoConvergence を返します。入力は変わらず、成功した結果は初期化済みです。

```rust
fn exp_with_prec (& self , prec : isize) -> Result < Self , DecimalError >
```

## 通常の使い方

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="log-exp-decimal-exp-with-prec" locale="ja" />

## 関連メソッド

[ln](./decimal-ln) · [log2](./decimal-log2) · [log10](./decimal-log10) · [exp](./decimal-exp)
