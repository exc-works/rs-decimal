---
locale: ja
source_revision: 001b8b003c099ab66879c97c5d63d2105fa698bcbc3c15c760670300d17a20a3
translation_provenance: en-US-japanese-reviewed
api_id: "ErrorKind::impl Eq"
---
# ErrorKind::impl Eq

Rust の Eq 実装を使います。

```rust
impl Eq for ErrorKind
```

## 引数、戻り値、通常の使い方

等価性はエラーメッセージと種類全体、または正確な列挙子を比較します。DecimalError の等価性は 1 種類への一致より厳密です。これはネイティブ Rust のトレイト構文で、ブラウザーはメソッドとして公開しません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-impl-eq" locale="ja" />

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
