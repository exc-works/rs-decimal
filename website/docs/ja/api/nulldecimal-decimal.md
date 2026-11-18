---
locale: ja
source_revision: 74688d1ee73b78a25c1792ba73a6a2e013084b0279376cef739a1b5c8cbed64f
translation_provenance: en-US-japanese-reviewed
api_id: NullDecimal.decimal
---
# NullDecimal.decimal

有効性とは独立して、保存された小数を確認または置換します。

```rust
decimal: Decimal
```

## 通常の使い方と意味

Rust は変更可能な Decimal フィールドを公開します。ブラウザーでは get_decimal_clone() で独立したコピーを取得し、set_decimal(value) で置換値を消費します。どちらも valid を変更しません。これらの明示的なアダプターにより、JavaScript に Rust の参照をそのまま提供するかのような誤解を避けます。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-nulldecimal-decimal" locale="ja" />

共通の例をネイティブ Rust とブラウザーで実行します。

## 関連ドキュメント

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
