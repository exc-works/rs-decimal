---
locale: ja
source_revision: 4b0752061c959ac5ce6e3b7af4bf7c9232d0d1cfb43869372b4caa7a5d6f49f4
translation_provenance: en-US-japanese-reviewed
api_id: NullDecimal::impl Debug
---
# NullDecimal::impl Debug

Rust の Debug で nullable の状態を確認します。

```rust
impl Debug for NullDecimal
```

## 通常の使い方と意味

診断用テキストには有効性と保存フィールドが含まれ、隠れた値も表示されます。安定した通信形式ではありません。ブラウザーではスナップショットを使います。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-nulldecimal-impl-debug" locale="ja" />

この Rust 構文の例はネイティブテストで実行します。ブラウザーでのトレイト対応を意味しません。

## 関連ドキュメント

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
