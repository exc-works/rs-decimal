---
locale: ja
source_revision: d3605c64b7e680ec58960c25358a321ed68015b81953ff878939d179147793d3
translation_provenance: en-US-japanese-reviewed
api_id: "DecimalError::impl Error"
---
# DecimalError::impl Error

Rust の Error 実装を使います。

```rust
impl Error for DecimalError
```

## 引数、戻り値、通常の使い方

ボックス化したエラーの伝播も含め、標準 Rust Error として使います。追加の種類は連鎖する原因ではなく、source() は None です。これはネイティブ Rust のトレイト構文で、ブラウザーはメソッドとして公開しません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-impl-error" locale="ja" />

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
