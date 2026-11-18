---
locale: ja
source_revision: d8d63d1c0e264a86d3f7608f1fd35c5b3c0175c1d05354d8cfc9826d8215e864
translation_provenance: en-US-japanese-reviewed
api_id: "Decimal::impl Neg::Output"
---
# Decimal::impl Neg::Output

借用する Rust Neg 構文を使います。

```rust
type Output = Decimal
```

## 通常の使い方と注意点

&Decimal に実装されているため、オペランドは引き続き使用できます。単項 - はスケールを保持し、既定のゼロも含め結果を初期化します。トレイトはコアの同じ名前のメソッドに委譲します。ブラウザーの JavaScript 演算子はオーバーロードされないため、bridge の add/sub/neg を使います。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-neg-output" locale="ja" />

## 実行

この項目はネイティブ Rust のトレイト構文であり、同名の JavaScript 演算子や SQLx オブジェクトを公開しません。既定のネイティブドキュメントテストでこの例を実行します。

## 関連メソッド

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
