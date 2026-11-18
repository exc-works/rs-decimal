---
locale: ja
source_revision: ce7339c2b8115fd42455e27eb624e03a51de3a069de71c01158ea752ddf6833a
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::bit_len
---
# Decimal::bit_len

スケール適用前の係数の絶対値のビット数を数えます。

```rust
fn bit_len (& self) -> usize
```

## 引数、戻り値、通常の使い方

引数なし。Rust は usize、bridge は number を返します。スケールは適用せず、符号ビットは数えません。-1.20 の係数は -120 で、ビット長は 7 です。既定のゼロを含むゼロには 0 を返します。最上位の 1 ビットの位置ではありません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-bit-len" locale="ja" />

## ブラウザーの制約

整数の転送は精度を失う JS number を経由しません。変換タプルは Rust の値／フラグの順序を保持します。false は正確な結果を保証しません。借用入力は変わらず、コンストラクターは結果を初期化します。[リソースポリシー](../resource-policy)はライブラリエラーとは別に、整数入力、出力スケール、有理数の係数増大を制限します。Rust のインポート構文はネイティブで検証するものとして明示します。

## 関連メソッド

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
