---
locale: ja
source_revision: cbc68f86b580dd7b41c6b741193a12b44e7d04cdf771d632d510adf82d84c1ba
translation_provenance: en-US-japanese-reviewed
api_id: most_significant_bit
---
# most_significant_bit

最上位の 1 ビットの位置を 0 始まりで求めます。

```rust
fn most_significant_bit (value : & BigInt) -> Result < usize , DecimalError >
```

## 引数、戻り値、通常の使い方

value: &BigInt を bigint で表します。Result<usize, DecimalError>/number を返します。0 と 1 はともに 0、120 は 6 です。負の入力は InvalidArgument です。これは自由関数で、Decimal メソッドではありません。対象が係数なら coefficient() を渡します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-most-significant-bit" locale="ja" />

## ブラウザーの制約

整数の転送は精度を失う JS number を経由しません。変換タプルは Rust の値／フラグの順序を保持します。false は正確な結果を保証しません。借用入力は変わらず、コンストラクターは結果を初期化します。[リソースポリシー](../resource-policy)はライブラリエラーとは別に、整数入力、出力スケール、有理数の係数増大を制限します。Rust のインポート構文はネイティブで検証するものとして明示します。

## 関連メソッド

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
