---
locale: ja
source_revision: 150ae4511881de6cce3960793291dd735b00fc7aa5175f86d5957a93c70d0c13
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::impl Debug
---
# Decimal::impl Debug

Rust のデバッグで小数を確認します。

```rust
impl Debug for Decimal
```

## 通常の使い方と意味

導出される診断用表現には内部フィールドが含まれ、安定したシリアライズ形式ではありません。ブラウザーの出力では明示的なスナップショットを使います。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-debug" locale="ja" />

この Rust 構文の例はネイティブテストで実行します。ブラウザーでのトレイト対応を意味しません。

## 関連ドキュメント

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
