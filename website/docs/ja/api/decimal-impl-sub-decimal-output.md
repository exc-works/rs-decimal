---
locale: ja
source_revision: c96f97d1831472af0c5d9cbfe01f313b01533a770c9ed531016eff9474a327fd
translation_provenance: en-US-japanese-reviewed
api_id: "Decimal::impl Sub < & Decimal >::Output"
---
# Decimal::impl Sub < & Decimal >::Output

借用する Rust Sub 構文を使います。

```rust
type Output = Decimal
```

## 通常の使い方と注意点

&Decimal に実装されているため、オペランドは引き続き使用できます。正確な結果は両オペランドの大きい方のスケールを使い、初期化済みです。関連型 Output は Decimal です。負の結果も許可し、safe_add/safe_sub の非負結果の要件は適用しません。トレイトはコアの同じ名前のメソッドに委譲します。ブラウザーの JavaScript 演算子はオーバーロードされないため、JavaScript の add/sub/neg を使います。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-sub-decimal-output" locale="ja" />

## 実行

この項目はネイティブ Rust のトレイト構文であり、同名の JavaScript 演算子や SQLx オブジェクトを公開しません。既定のネイティブドキュメントテストでこの例を実行します。

## 関連メソッド

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
