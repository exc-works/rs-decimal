---
locale: ja
source_revision: bceeb7ede6fc6a0450eec9bb9848cbe43ca4c0d990bf02edf9131f531001c34f
translation_provenance: en-US-japanese-reviewed
api_id: "DecimalError::impl PartialEq"
---
# DecimalError::impl PartialEq

Rust の PartialEq 実装を使います。

```rust
impl PartialEq for DecimalError
```

## 引数、戻り値、通常の使い方

等価性はエラーメッセージと種類全体、または正確な列挙子を比較します。DecimalError の等価性は 1 種類への一致より厳密です。これはネイティブ Rust のトレイト構文で、ブラウザーはメソッドとして公開しません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-impl-partialeq" locale="ja" />

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
