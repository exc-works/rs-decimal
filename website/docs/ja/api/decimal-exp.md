---
locale: ja
source_revision: cc1566925b96baa324fb64a51eba19d80177bb315f68b0f6a9ee1d05c6da9807
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::exp
---
# Decimal::exp

指数関数 e^xを計算します。Rust は Result<Decimal, DecimalError>、bridge は Decimal または構造化された例外を返します。引数なし。既定の出力精度は max(入力の小数点以下の桁数, 20) であり、有効数字 20 桁ではありません。コアではすべての実数入力を許可します。exp(0) は初期化済みの 1 です。十分に負の引数は、要求精度で初期化済みゼロへ正しく丸められる場合があります。コアは値を囲む区間を使い、丸めを証明できたときだけ結果を受理します。mode 引数のない最近接偶数丸めです。コアの上限内で証明できない場合は、未検証の近似値ではなく NoConvergence を返します。入力は変わらず、成功した結果は初期化済みです。

```rust
fn exp (& self) -> Result < Self , DecimalError >
```

## 通常の使い方

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="log-exp-decimal-exp" locale="ja" />

## 関連メソッド

[ln](./decimal-ln) · [log2](./decimal-log2) · [log10](./decimal-log10) · [exp](./decimal-exp)
