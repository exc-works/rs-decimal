---
locale: ja
source_revision: ed7eae766562aad92ba36a1a91ecd3051f3e73486fdc4f475516ea974adac927
translation_provenance: en-US-japanese-reviewed
api_id: "Decimal::impl fmt :: Display::fmt"
---
# Decimal::impl fmt :: Display::fmt

Rust の書式指定構文で小数を表示します。

```rust
fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result
```

## 通常の使い方と意味

format!("{}", value)、println!、to_string() を使います。fmt::Formatter は書式処理の仕組みから渡されます。既定の書式では小数部分の末尾のゼロを除去します。追加の書式フラグは書式設定グループで説明します。bridge では string()/string_with_trailing_zeros() を使います。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-fmt-display-fmt" locale="ja" />

この Rust 構文の例はネイティブテストで実行します。ブラウザーでのトレイト対応を意味しません。

## 関連ドキュメント

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
