---
locale: ja
source_revision: b8a64fe28faf9f699b61b2f86e331fd9c5b8333882ce85a4476ea07f56d57e1d
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::impl FromStr::from_str
---
# Decimal::impl FromStr::from_str

Rust の parse と FromStr で小数テキストを解析します。

```rust
fn from_str (text : & str) -> Result < Self , Self :: Err >
```

## 通常の使い方と意味

Decimal または DecimalError を返します。構文とゼロの正規化は new_from_string と同じです。ブラウザーはその名前付きメソッドを呼び出します。Rust トレイト構文はネイティブ専用です。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-fromstr-from-str" locale="ja" />

この Rust 構文の例はネイティブテストで実行します。ブラウザーでのトレイト対応を意味しません。

## 関連ドキュメント

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
