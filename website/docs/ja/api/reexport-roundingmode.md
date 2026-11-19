---
locale: ja
source_revision: 73057fa1830878a540ff385b7411b32617a1d65647f7c283e5495a0cce3aca84
translation_provenance: en-US-japanese-reviewed
api_id: reexport::RoundingMode
---
# reexport::RoundingMode

クレートルートから丸めモードをインポートします。

```rust
pub use rounding::RoundingMode as RoundingMode
```

## 引数、戻り値、通常の使い方

rs_decimal::RoundingMode を使います。同じ列挙型であり、別の実装ではありません。ブラウザーは Rust のインポート構文を使わず、名前付きの RoundingMode オブジェクトを公開します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-reexport-roundingmode" locale="ja" />

## 関連メソッド

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
