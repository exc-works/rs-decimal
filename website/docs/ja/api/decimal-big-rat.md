---
locale: ja
source_revision: b417c8ff7bf7c7086d2d314348cd490ed506bea3f1d87b65cb817d90defb82d7
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::big_rat
---
# Decimal::big_rat

約分した正確な有理数値を読み取ります。

```rust
fn big_rat (& self) -> BigRational
```

## 引数、戻り値、通常の使い方

引数なし。Rust は BigRational を返します。bridge は分母を正に正規化した、不変の {numerator: bigint, denominator: bigint} スナップショットを返します。小数のスケールや末尾のゼロは有理数のメタデータとして保持されず、ゼロは 0/1 になります。入力は借用し、変更しません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-big-rat" locale="ja" />

## ブラウザーの制約

整数の転送は精度を失う JS number を経由しません。変換タプルは Rust の値／フラグの順序を保持します。false は正確な結果を保証しません。借用入力は変わらず、コンストラクターは結果を初期化します。[リソースポリシー](../resource-policy)はライブラリエラーとは別に、整数入力、出力スケール、有理数の係数増大を制限します。Rust のインポート構文はネイティブで検証するものとして明示します。

## 関連メソッド

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
