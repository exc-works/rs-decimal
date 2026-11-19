---
locale: ja
source_revision: 536ad1418de3fbdb10070d55eda3a68aef79b3b4a2bc3507ce307a0421f5b595
translation_provenance: en-US-japanese-reviewed
api_id: RoundingMode
---
# RoundingMode

7 つの明示的な丸め規則から選択します。

```rust
pub enum RoundingMode
```

## 引数、戻り値、通常の使い方

Rust では名前付き列挙子、JavaScript では RoundingMode のフィールドを使います。Rust の判別値 0–6 はブラウザーの数値と一致します。Down はゼロ方向、Up はゼロから遠ざかる方向、Ceiling は正の無限大方向です。HalfUp/HalfDown/HalfEven はちょうど中間のとき、それぞれゼロから遠ざかる方向／ゼロ方向／偶数を選びます。Unnecessary は正確さを要求します。暗黙のグローバルな丸め設定はありません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode" locale="ja" />

## 関連メソッド

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
