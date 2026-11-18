---
locale: ja
source_revision: 32a89fd803ef33f414f98138648ca1bbfe7cf03c12443bee715ec01ffb382e0d
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::floor
---
# Decimal::floor

負の無限大方向へ丸めて整数にします。

```rust
fn floor (& self) -> Self
```

## 引数、戻り値、通常の使い方

引数なし。スケール 0 の新しい Decimal を返します。負の値では、ゼロ方向の切り捨てと floor は異なります。-1.5 は前者では -1、floor では -2 になります。入力は借用され、変更しません。既定のゼロが入力でも結果は初期化済みです。精度を上げると数値を変えずにゼロを追加します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-floor" locale="ja" />

## ブラウザーの制約

整数引数は wasm32 の範囲内である必要があります。ブラウザーはさらに[リソースポリシー](../resource-policy)を適用し、指定スケール、係数の増大、呼び出し回数を制限します。リソースエラーと数値ライブラリのエラーは区別されます。借用メソッドは入力を消費しません。Rust 専用のトレイト構文はネイティブで検証するものとして明示します。

## 関連メソッド

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
