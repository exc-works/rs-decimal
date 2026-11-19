---
locale: ja
source_revision: 53cafbe90f3ab017866bcf2afb140d94d6b1a293f7aa6ca2a32a775110c8f82c
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::ceil
---
# Decimal::ceil

正の無限大方向へ丸めて整数にします。

```rust
fn ceil (& self) -> Self
```

## 引数、戻り値、通常の使い方

引数なし。スケール 0 の新しい Decimal を返します。入力は借用され、変更しません。既定のゼロが入力でも結果は初期化済みです。精度を上げると数値を変えずにゼロを追加します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-ceil" locale="ja" />

## 関連メソッド

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
