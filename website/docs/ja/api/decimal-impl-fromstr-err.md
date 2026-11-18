---
locale: ja
source_revision: 45b2b4aa308ba6b40046aead6dddbc34fd9a06d38be18fb996c0bc76cff67656
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::impl FromStr::Err
---
# Decimal::impl FromStr::Err

Decimal の FromStr 実装が返すエラー型を示します。

```rust
type Err = DecimalError
```

## 通常の使い方と意味

関連型 `<Decimal as FromStr>::Err` は `DecimalError` です。不正なテキストではこのエラーを返し、パニックしません。エラーの種類を調べると、不正な構文と他の失敗を区別できます。これは Rust の型エイリアスであり、JavaScript コンストラクターではありません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-fromstr-err" locale="ja" />

この Rust 構文の例はネイティブテストで実行します。ブラウザーでのトレイト対応を意味しません。

## 関連ドキュメント

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
