---
locale: ja
source_revision: 19e6388558d46ddb3598af453ea4bcd2872fc2f6ce5d4c459e0dfbab37170bfa
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::log2
---
# Decimal::log2

底 2 の対数を計算します。引数なし。既定の出力精度は max(入力の小数点以下の桁数, 20) であり、有効数字 20 桁ではありません。入力は正である必要があり、ゼロや負の入力は InvalidLog を返します。底の正確な整数乗の対数は正確な整数となり、出力スケールで保持します。コアは値を囲む区間を使い、丸めを証明できたときだけ結果を受理します。mode 引数のない最近接偶数丸めです。コアの上限内で証明できない場合は、未検証の近似値ではなく NoConvergence を返します。入力は変わらず、成功した結果は初期化済みです。

```rust
fn log2 (& self) -> Result < Self , DecimalError >
```

## 通常の使い方

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="log-exp-decimal-log2" locale="ja" />

## 関連メソッド

[ln](./decimal-ln) · [log2](./decimal-log2) · [log10](./decimal-log10) · [exp](./decimal-exp)
