---
locale: ja
source_revision: 97b85000fbf7a0fb6537eae68258f927a7739647e41e96de3c0f26f460f2d35a
translation_provenance: en-US-japanese-reviewed
api_id: "Decimal::impl fmt :: Display"
---
# Decimal::impl fmt :: Display

Rust の書式指定構文で小数を表示します。

```rust
impl fmt :: Display for Decimal
```

## 通常の使い方と意味

format!("{}", value)、println!、to_string() を使います。fmt::Formatter は書式処理の仕組みから渡されます。既定の書式では小数部分の末尾のゼロを除去します。追加の書式フラグは書式設定グループで説明します。JavaScript では string()/string_with_trailing_zeros() を使います。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-fmt-display" locale="ja" />

この Rust 構文の例はネイティブテストで実行します。ブラウザーでのトレイト対応を意味しません。

## 関連ドキュメント

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
