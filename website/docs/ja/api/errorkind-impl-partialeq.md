---
locale: ja
source_revision: 5ee0e7b51aa068361991bd8a8b6016ae9f25f271ec57ec1d0990b39445f68b64
translation_provenance: en-US-japanese-reviewed
api_id: "ErrorKind::impl PartialEq"
---
# ErrorKind::impl PartialEq

Rust の PartialEq 実装を使います。

```rust
impl PartialEq for ErrorKind
```

## 引数、戻り値、通常の使い方

等価性はエラーメッセージと種類全体、または正確な列挙子を比較します。DecimalError の等価性は 1 種類への一致より厳密です。これはネイティブ Rust のトレイト構文で、ブラウザーはメソッドとして公開しません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-impl-partialeq" locale="ja" />

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
