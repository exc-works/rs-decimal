---
locale: ja
source_revision: fbc64b05e8340067582332c66c6f66973b00c5a17412c42b00f322b92b77d915
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::ln_with_prec
---
# Decimal::ln_with_prec

自然対数を計算します。prec: 非負の isize を整数 number で表し、小数点以下の桁数を選びます。負の精度は定義域の検査前に InvalidPrecision を返します。入力は正である必要があり、ゼロや負の入力は InvalidLog を返します。ln(1) は正確にゼロです。コアは値を囲む区間を使い、丸めを証明できたときだけ結果を受理します。mode 引数のない最近接偶数丸めです。コアの上限内で証明できない場合は、未検証の近似値ではなく NoConvergence を返します。入力は変わらず、成功した結果は初期化済みです。

```rust
fn ln_with_prec (& self , prec : isize) -> Result < Self , DecimalError >
```

## 通常の使い方

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="log-exp-decimal-ln-with-prec" locale="ja" />

## 関連メソッド

[ln](./decimal-ln) · [log2](./decimal-log2) · [log10](./decimal-log10) · [exp](./decimal-exp)
