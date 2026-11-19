---
locale: ja
source_revision: cbf84996c156999f0cfb9cac49a1135ecf0d19ca8aba3abbedbed0acff443c0a
translation_provenance: en-US-japanese-reviewed
api_id: "DecimalError::impl Eq"
---
# DecimalError::impl Eq

Rust の Eq 実装を使います。

```rust
impl Eq for DecimalError
```

## 引数、戻り値、通常の使い方

等価性はエラーメッセージと種類全体、または正確な列挙子を比較します。DecimalError の等価性は 1 種類への一致より厳密です。これはネイティブ Rust のトレイト構文で、ブラウザーはメソッドとして公開しません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-impl-eq" locale="ja" />

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
