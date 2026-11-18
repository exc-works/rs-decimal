---
locale: ja
source_revision: 056afbd5297b8421283bf4270ff73e28ea9129becd2598b31ff7a84ee04dacd5
translation_provenance: en-US-japanese-reviewed
api_id: NullDecimal::impl Clone
---
# NullDecimal::impl Clone

Rust で有効性と保存された小数の両方を複製します。

```rust
impl Clone for NullDecimal
```

## 通常の使い方と意味

valid が false でも、複製時には隠れた値を保持します。これはネイティブ Rust トレイトであり、変更を追跡する共有参照ではありません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-nulldecimal-impl-clone" locale="ja" />

この Rust 構文の例はネイティブテストで実行します。ブラウザーでのトレイト対応を意味しません。

## 関連ドキュメント

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
