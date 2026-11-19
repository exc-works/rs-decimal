---
locale: ja
source_revision: b1efdab89860dbce37ea848eed30ac657af5e945bd5643b2378d7d893a2ce9a2
translation_provenance: en-US-japanese-reviewed
api_id: RoundingMode::from_raw
---
# RoundingMode::from_raw

整数の丸めモードコードを検証します。

```rust
fn from_raw (value : i32) -> Result < Self , DecimalError >
```

## 引数、戻り値、通常の使い方

value: i32/整数 number。0–6 には対応するモード、それ以外の i32 には InvalidArgument を返します。ブラウザーで小数、非有限値、i32 範囲外の値を渡すと、Rust の前に引数検証で失敗します。JavaScript は Rust の検証器を呼び出し、数値のモードを返します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-from-raw" locale="ja" />

## 関連メソッド

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
