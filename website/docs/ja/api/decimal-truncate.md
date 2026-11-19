---
locale: ja
source_revision: d24f0cd3d78b3594804e8def855ad76024ffbd370d84b3931d49427d7730909f
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::truncate
---
# Decimal::truncate

ゼロ方向へ丸めて整数にします。

```rust
fn truncate (& self) -> Self
```

## 引数、戻り値、通常の使い方

引数なし。スケール 0 の新しい Decimal を返します。負の値では、ゼロ方向の切り捨てと floor は異なります。-1.5 は前者では -1、floor では -2 になります。入力は借用され、変更しません。既定のゼロが入力でも結果は初期化済みです。精度を上げると数値を変えずにゼロを追加します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-truncate" locale="ja" />

## 関連メソッド

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
