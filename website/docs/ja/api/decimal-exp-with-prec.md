---
locale: ja
source_revision: 4817d408a3e6bcf6471a527b25ba77c1f63c6984d0953a16de68bfe470c82e92
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

## ブラウザーの制約

このグループには追加の[リソース制限](../resource-policy)があります。出力精度は最大 128、入力スケールは最大 256、入力係数は最大 4096 ビットです。通常の exp 計算では入力の絶対値も最大 64 です。コアが直接ゼロに丸められると証明できる負の入力は高速経路を保持します。入力スケールが大きいと、引数なしのメソッドが 128 を超える既定精度を選ぶ場合があります。with_prec で許可された出力精度を明示してください。リソースエラーとコアの NoConvergence は区別します。Stop は実行を中断し、次の実行では新しい Wasm インスタンスを作ります。

## 関連メソッド

[ln](./decimal-ln) · [log2](./decimal-log2) · [log10](./decimal-log10) · [exp](./decimal-exp)
