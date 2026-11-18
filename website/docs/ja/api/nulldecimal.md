---
locale: ja
source_revision: 8675e68949343d33893aa6f81cc6a57f32cdafd3aaf80161520178113f46a31d
translation_provenance: en-US-japanese-reviewed
api_id: NullDecimal
---
# NullDecimal

有効性と保存値が独立した、任意の小数値を表現します。

```rust
pub struct NullDecimal
```

## 通常の使い方と意味

値の有無は valid だけで決まります。無効な nullable がゼロ以外の小数を保持することも、有効な nullable が未初期化のゼロを含むこともあります。公開 Rust decimal フィールドは、JavaScript ではコピーとムーブを明示したアダプターを通じて公開します。既定値は値なしです。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-nulldecimal" locale="ja" />

共通の例をネイティブ Rust とブラウザーで実行します。

## 関連ドキュメント

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
