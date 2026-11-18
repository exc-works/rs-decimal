---
locale: ja
source_revision: 317e1f4c19916312bb2823fc0a526f5311afada3dc44c8d3247bdd5fc0f08111
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::impl Ord::cmp
---
# Decimal::impl Ord::cmp

Rust の Ord で小数を数値順に比較します。

```rust
fn cmp (& self , other : & Self) -> Ordering
```

## 通常の使い方と意味

Ord::cmp は Ordering を返し、-1/0/1 を返す Decimal::cmp とは異なります。表現の違いは順序に影響しません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-ord-cmp" locale="ja" />

この Rust 構文の例はネイティブテストで実行します。ブラウザーでのトレイト対応を意味しません。

## 関連ドキュメント

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
