---
locale: ja
source_revision: dfbccd4ef90aecedf2d24565d11016e8068251bd103af1f82ea5d3b25f25cee0
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::round
---
# Decimal::round

最近接偶数丸めで整数にします。

```rust
fn round (& self) -> Self
```

## 引数、戻り値、通常の使い方

引数なし。スケール 0 の新しい Decimal を返します。ここでは最近接偶数丸めが既定です。JavaScript の Math.round の中間値処理とは異なります。入力は借用され、変更しません。既定のゼロが入力でも結果は初期化済みです。精度を上げると数値を変えずにゼロを追加します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-round" locale="ja" />

## ブラウザーの制約

整数引数は wasm32 の範囲内である必要があります。ブラウザーはさらに[リソースポリシー](../resource-policy)を適用し、指定スケール、係数の増大、呼び出し回数を制限します。リソースエラーと数値ライブラリのエラーは区別されます。借用メソッドは入力を消費しません。Rust 専用のトレイト構文はネイティブで検証するものとして明示します。

## 関連メソッド

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
