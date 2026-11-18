---
locale: ja
source_revision: 24579cc5f98ad85ad1558995873e9949e2e4e6ae6e01cc4702b2ee7ca18a14ea
translation_provenance: en-US-japanese-reviewed
api_id: RoundingMode
---
# RoundingMode

7 つの明示的な丸め規則から選択します。

```rust
pub enum RoundingMode
```

## 引数、戻り値、通常の使い方

Rust では名前付き列挙子、bridge では RoundingMode のフィールドを使います。Rust の判別値 0–6 はブラウザーの数値と一致します。Down はゼロ方向、Up はゼロから遠ざかる方向、Ceiling は正の無限大方向です。HalfUp/HalfDown/HalfEven はちょうど中間のとき、それぞれゼロから遠ざかる方向／ゼロ方向／偶数を選びます。Unnecessary は正確さを要求します。暗黙のグローバルな丸め設定はありません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode" locale="ja" />

## ブラウザーの制約

整数引数は wasm32 の範囲内である必要があります。ブラウザーはさらに[リソースポリシー](../resource-policy)を適用し、指定スケール、係数の増大、呼び出し回数を制限します。リソースエラーと数値ライブラリのエラーは区別されます。借用メソッドは入力を消費しません。Rust 専用のトレイト構文はネイティブで検証するものとして明示します。

## 関連メソッド

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
