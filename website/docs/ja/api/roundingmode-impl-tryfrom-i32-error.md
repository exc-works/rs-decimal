---
locale: ja
source_revision: f397dc1220fc731959fe63aa3e5a1c46b0d68942bea1f3f5f567773d91995a1b
translation_provenance: en-US-japanese-reviewed
api_id: "RoundingMode::impl TryFrom < i32 >::Error"
---
# RoundingMode::impl TryFrom < i32 >::Error

TryFrom で生の丸め判別値を検証します。

```rust
type Error = DecimalError
```

## 通常の使い方と注意点

value: i32。`Result<RoundingMode, DecimalError>` を返します。コード 0–6 は 5=HalfEven を含む固定のモード対応を使います。不明なコードはパニックせず InvalidArgument を返します。関連型 Error は DecimalError です。from_raw に委譲し、ブラウザーは厳密な i32 引数検査を行う RoundingMode.from_raw(number) を使います。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-roundingmode-impl-tryfrom-i32-error" locale="ja" />

## 実行

この項目はネイティブ Rust のトレイト構文であり、同名の JavaScript 演算子や SQLx オブジェクトを公開しません。既定のネイティブドキュメントテストでこの例を実行します。

## 関連メソッド

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
