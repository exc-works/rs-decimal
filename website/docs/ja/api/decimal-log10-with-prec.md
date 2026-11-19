---
locale: ja
source_revision: 7350b5ad5bfd585c249094865cd49a6bad71c645ac5e1b3b72695308a1cc7d0a
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::log10_with_prec
---
# Decimal::log10_with_prec

底 10 の対数を計算します。Rust は Result<Decimal, DecimalError>、bridge は Decimal または構造化された例外を返します。prec: 非負の isize を整数 number で表し、小数点以下の桁数を選びます。負の精度は定義域の検査前に InvalidPrecision を返します。入力は正である必要があり、ゼロや負の入力は InvalidLog を返します。底の正確な整数乗の対数は正確な整数となり、出力スケールで保持します。コアは値を囲む区間を使い、丸めを証明できたときだけ結果を受理します。mode 引数のない最近接偶数丸めです。コアの上限内で証明できない場合は、未検証の近似値ではなく NoConvergence を返します。入力は変わらず、成功した結果は初期化済みです。

```rust
fn log10_with_prec (& self , prec : isize) -> Result < Self , DecimalError >
```

## 通常の使い方

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="log-exp-decimal-log10-with-prec" locale="ja" />

## 関連メソッド

[ln](./decimal-ln) · [log2](./decimal-log2) · [log10](./decimal-log10) · [exp](./decimal-exp)
