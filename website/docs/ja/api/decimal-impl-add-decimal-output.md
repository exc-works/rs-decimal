---
locale: ja
source_revision: 70d6ac36cf125e6927984240187236d52728f3fe02ff25733f35872ac71b904c
translation_provenance: en-US-japanese-reviewed
api_id: "Decimal::impl Add < & Decimal >::Output"
---
# Decimal::impl Add < & Decimal >::Output

借用する Rust Add 構文を使います。

```rust
type Output = Decimal
```

## 通常の使い方と注意点

&Decimal に実装されているため、オペランドは引き続き使用できます。正確な結果は両オペランドの大きい方のスケールを使い、初期化済みです。関連型 Output は Decimal です。負の結果も許可し、safe_add/safe_sub の非負結果の要件は適用しません。トレイトはコアの同じ名前のメソッドに委譲します。ブラウザーの JavaScript 演算子はオーバーロードされないため、bridge の add/sub/neg を使います。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-add-decimal-output" locale="ja" />

## 実行

この項目はネイティブ Rust のトレイト構文であり、同名の JavaScript 演算子や SQLx オブジェクトを公開しません。既定のネイティブドキュメントテストでこの例を実行します。

## 関連メソッド

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
