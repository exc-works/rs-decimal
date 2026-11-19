---
locale: ja
source_revision: c654c9726a283aeb3b59ec2a73dcd6f45d404c2677f244d97a60cf2ff73a642b
translation_provenance: en-US-japanese-reviewed
api_id: NullDecimal::impl Default
---
# NullDecimal::impl Default

Rust の Default で値のない nullable を作成します。

```rust
impl Default for NullDecimal
```

## 通常の使い方と意味

有効性と内部の初期化状態はともに false です。JavaScript は NullDecimal.default() を提供します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-nulldecimal-impl-default" locale="ja" />

この Rust 構文の例はネイティブテストで実行します。ブラウザーでのトレイト対応を意味しません。

## 関連ドキュメント

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
