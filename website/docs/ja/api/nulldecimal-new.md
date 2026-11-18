---
locale: ja
source_revision: 6d83b155d9084cec9f42deea0ac618a5403b16dd0003e0789794c0428f3a019a
translation_provenance: en-US-japanese-reviewed
api_id: NullDecimal::new
---
# NullDecimal::new

小数をムーブし、値のある nullable にします。

```rust
fn new (decimal : Decimal) -> Self
```

## 通常の使い方と意味

decimal: Decimal を消費します。decimal が未初期化のゼロでも結果は常に valid=true です。ムーブされた JavaScript 入力は使用できなくなります。元の値が必要なら複製してください。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-nulldecimal-new" locale="ja" />

共通の例をネイティブ Rust とブラウザーで実行します。

## 関連ドキュメント

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
