---
locale: ja
source_revision: cd679689311c552a10e0f46bebe2c6d4b3b7c9ae648dc393862458b144cd8b20
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::ln_with_prec
---
# Decimal::ln_with_prec

自然対数を計算します。Rust は Result<Decimal, DecimalError>、bridge は Decimal または構造化された例外を返します。prec: 非負の isize を整数 number で表し、小数点以下の桁数を選びます。負の精度は定義域の検査前に InvalidPrecision を返します。入力は正である必要があり、ゼロや負の入力は InvalidLog を返します。ln(1) は正確にゼロです。コアは値を囲む区間を使い、丸めを証明できたときだけ結果を受理します。mode 引数のない最近接偶数丸めです。コアの上限内で証明できない場合は、未検証の近似値ではなく NoConvergence を返します。入力は変わらず、成功した結果は初期化済みです。

```rust
fn ln_with_prec (& self , prec : isize) -> Result < Self , DecimalError >
```

## 通常の使い方

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="log-exp-decimal-ln-with-prec" locale="ja" />

## ブラウザーの制約

このグループには追加の[リソース制限](../resource-policy)があります。出力精度は最大 128、入力スケールは最大 256、入力係数は最大 4096 ビットです。通常の exp 計算では入力の絶対値も最大 64 です。コアが直接ゼロに丸められると証明できる負の入力は高速経路を保持します。入力スケールが大きいと、引数なしのメソッドが 128 を超える既定精度を選ぶ場合があります。with_prec で許可された出力精度を明示してください。リソースエラーとコアの NoConvergence は区別します。Stop は実行を中断し、次の実行では新しい Wasm インスタンスを作ります。

## 関連メソッド

[ln](./decimal-ln) · [log2](./decimal-log2) · [log10](./decimal-log10) · [exp](./decimal-exp)
