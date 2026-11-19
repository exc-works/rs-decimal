---
locale: ja
source_revision: f67e1caa0fb769c278b4d8058441646a0cf632963d9da4fd8b19596b64289dc8
translation_provenance: en-US-japanese-reviewed
api_id: reexport::BigRational
---
# reexport::BigRational

クレートルートから BigRational をインポートします。

```rust
pub use num_rational::BigRational as BigRational
```

## 引数、戻り値、通常の使い方

これは公開 Rust 再エクスポートであり、新しい型や実装ではありません。インポート構文はネイティブで検証します。ブラウザーではそれぞれ bigint、正確な有理数のフィールドオブジェクト、グローバル関数 most_significant_bit に対応します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-reexport-bigrational" locale="ja" />

## 関連メソッド

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
