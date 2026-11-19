---
locale: ja
source_revision: 6155fe36a4ec8e1b03a89c20c5c7247a068af20681f3b463de98b38c447a2097
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::significant_figures
---
# Decimal::significant_figures

指定した最大有効桁数へ丸めます。

```rust
fn significant_figures (& self , figures : isize , mode : RoundingMode ,) -> Result < Self , DecimalError >
```

## 引数、戻り値、通常の使い方

figures: 正の isize/整数 number。mode は丸め規則です。小数点以下の桁数とは異なり、有効数字は係数の桁を数えます。精度を追加することはなく、整数部分も丸める場合があります（有効数字 3 桁で 1234.5 → 1230）。figures が非正なら InvalidArgument、Unnecessary で丸めが必要なら RoundUnnecessary を返します。係数の桁上がりの可能性をブラウザーの上限と事前に照合します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-significant-figures" locale="ja" />

## 関連メソッド

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
