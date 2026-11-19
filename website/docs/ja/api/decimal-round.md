---
locale: ja
source_revision: 6e97ceecb692f3b30f694df6cf3812a7bc93e427ba0e4c690447e33c151380a9
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

## 関連メソッド

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
