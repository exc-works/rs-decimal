---
locale: ja
source_revision: 37fcbc194399d8059373fc9c0c61833d997ffb41871435eb1c6867b7870d1acc
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::impl Clone
---
# Decimal::impl Clone

表現全体を保持して小数を複製します。

```rust
impl Clone for Decimal
```

## 通常の使い方と意味

Clone は係数、スケール、初期化状態をコピーします。JavaScript では Rust トレイトではなく、名前付きの clone() アダプターを使います。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-clone" locale="ja" />

この Rust 構文の例はネイティブテストで実行します。ブラウザーでのトレイト対応を意味しません。

## 関連ドキュメント

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
