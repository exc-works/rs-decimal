---
locale: ja
source_revision: 65703feb0e11af9dfe348c058c514dbf10ab13c1d2a720632926e57fee628fa9
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::impl FromStr
---
# Decimal::impl FromStr

Rust の parse と FromStr で小数テキストを解析します。

```rust
impl FromStr for Decimal
```

## 通常の使い方と意味

Decimal または DecimalError を返します。構文とゼロの正規化は new_from_string と同じです。ブラウザーはその名前付きメソッドを呼び出します。Rust トレイト構文はネイティブ専用です。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-fromstr" locale="ja" />

この Rust 構文の例はネイティブテストで実行します。ブラウザーでのトレイト対応を意味しません。

## 関連ドキュメント

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
