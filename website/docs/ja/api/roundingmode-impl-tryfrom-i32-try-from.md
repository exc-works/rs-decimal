---
locale: ja
source_revision: 584a930ae7d128f9ab20d43eb7ffff2b455374bd06ee0e33745542254e1599b0
translation_provenance: en-US-japanese-reviewed
api_id: "RoundingMode::impl TryFrom < i32 >::try_from"
---
# RoundingMode::impl TryFrom < i32 >::try_from

TryFrom で生の丸め判別値を検証します。

```rust
fn try_from (value : i32) -> Result < Self , Self :: Error >
```

## 通常の使い方と注意点

value: i32。`Result<RoundingMode, DecimalError>` を返します。コード 0–6 は 5=HalfEven を含む固定のモード対応を使います。不明なコードはパニックせず InvalidArgument を返します。関連型 Error は DecimalError です。from_raw に委譲し、ブラウザーは厳密な i32 引数検査を行う RoundingMode.from_raw(number) を使います。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-roundingmode-impl-tryfrom-i32-try-from" locale="ja" />

## 実行

この項目はネイティブ Rust のトレイト構文であり、同名の JavaScript 演算子や SQLx オブジェクトを公開しません。既定のネイティブドキュメントテストでこの例を実行します。

## 関連メソッド

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
