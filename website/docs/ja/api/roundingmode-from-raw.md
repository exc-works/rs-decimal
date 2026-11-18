---
locale: ja
source_revision: 16147351e8fe026905ac983886e4a94a6ce341eb3c301ee1901cd0719ea82ee1
translation_provenance: en-US-japanese-reviewed
api_id: RoundingMode::from_raw
---
# RoundingMode::from_raw

整数の丸めモードコードを検証します。

```rust
fn from_raw (value : i32) -> Result < Self , DecimalError >
```

## 引数、戻り値、通常の使い方

value: i32/整数 number。0–6 には対応するモード、それ以外の i32 には InvalidArgument を返します。ブラウザーで小数、非有限値、i32 範囲外の値を渡すと、Rust の前に引数検証で失敗します。bridge は Rust の検証器を呼び出し、数値のモードを返します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-from-raw" locale="ja" />

## ブラウザーの制約

整数引数は wasm32 の範囲内である必要があります。ブラウザーはさらに[リソースポリシー](../resource-policy)を適用し、指定スケール、係数の増大、呼び出し回数を制限します。リソースエラーと数値ライブラリのエラーは区別されます。借用メソッドは入力を消費しません。Rust 専用のトレイト構文はネイティブで検証するものとして明示します。

## 関連メソッド

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
