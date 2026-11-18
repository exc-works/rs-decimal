---
locale: ja
source_revision: fc360056fa6a06d0df76e11b3c2a65adb20ebcb5dcc14e8a0ad1eefc1447a90e
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::impl Ord
---
# Decimal::impl Ord

Rust の Ord で小数を数値順に比較します。

```rust
impl Ord for Decimal
```

## 通常の使い方と意味

Ord::cmp は Ordering を返し、-1/0/1 を返す Decimal::cmp とは異なります。表現の違いは順序に影響しません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-ord" locale="ja" />

この Rust 構文の例はネイティブテストで実行します。ブラウザーでのトレイト対応を意味しません。

## 関連ドキュメント

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
