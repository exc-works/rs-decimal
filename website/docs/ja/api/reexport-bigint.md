---
locale: ja
source_revision: 8110d2fd30808fd737c3f5730a5be2d19cf89f407f5d688859de476689342458
translation_provenance: en-US-japanese-reviewed
api_id: reexport::BigInt
---
# reexport::BigInt

クレートルートから BigInt をインポートします。

```rust
pub use num_bigint::BigInt as BigInt
```

## 引数、戻り値、通常の使い方

これは公開 Rust 再エクスポートであり、新しい型や実装ではありません。インポート構文はネイティブで検証します。ブラウザーではそれぞれ bigint、正確な有理数のフィールドオブジェクト、グローバル関数 most_significant_bit に対応します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-reexport-bigint" locale="ja" />

## ブラウザーの制約

整数の転送は精度を失う JS number を経由しません。変換タプルは Rust の値／フラグの順序を保持します。false は正確な結果を保証しません。借用入力は変わらず、コンストラクターは結果を初期化します。[リソースポリシー](../resource-policy)はライブラリエラーとは別に、整数入力、出力スケール、有理数の係数増大を制限します。Rust のインポート構文はネイティブで検証するものとして明示します。

## 関連メソッド

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
