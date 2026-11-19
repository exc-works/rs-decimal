---
locale: ja
source_revision: 6440a61a644ecdbf15d2da0254f2f46ec6a0bac106e6a427225c0f45a05d250f
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::must_non_negative
---
# Decimal::must_non_negative

非負であることを要求し、初期化済みのコピーを返します。

```rust
fn must_non_negative (& self) -> Self
```

## 引数、戻り値、通常の使い方

引数なし。ゼロを含む非負の入力では Decimal を返します。負の入力では Rust は文字列ペイロードでパニックします。Wasm では回復可能な DecimalError ではなくトラップとなり、playground の実行が終了します。新しい実行で Wasm の状態を作り直してください。これは数値の表明であり、絶対値への変換ではありません。符号の変更には abs を使います。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-must-non-negative" locale="ja" />

## 関連メソッド

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
