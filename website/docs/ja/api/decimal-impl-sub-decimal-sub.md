---
locale: ja
source_revision: c4a0a7eefa609b0c57e86546936b34ea64a5ece51d5f57a28f19151facde5f69
translation_provenance: en-US-japanese-reviewed
api_id: "Decimal::impl Sub < & Decimal >::sub"
---
# Decimal::impl Sub < & Decimal >::sub

借用する Rust Sub 構文を使います。

```rust
fn sub (self , other : & Decimal) -> Decimal
```

## 通常の使い方と注意点

&Decimal に実装されているため、オペランドは引き続き使用できます。正確な結果は両オペランドの大きい方のスケールを使い、初期化済みです。関連型 Output は Decimal です。負の結果も許可し、safe_add/safe_sub の非負結果の要件は適用しません。トレイトはコアの同じ名前のメソッドに委譲します。ブラウザーの JavaScript 演算子はオーバーロードされないため、JavaScript の add/sub/neg を使います。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-sub-decimal-sub" locale="ja" />

## 実行

この項目はネイティブ Rust のトレイト構文であり、同名の JavaScript 演算子や SQLx オブジェクトを公開しません。既定のネイティブドキュメントテストでこの例を実行します。

## 関連メソッド

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
