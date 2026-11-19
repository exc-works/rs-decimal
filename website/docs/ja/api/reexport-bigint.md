---
locale: ja
source_revision: f07542c46478fd9edd09bbf3af9bd03d15608eb33e9ed99beb51b022df098d6a
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

## 関連メソッド

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
