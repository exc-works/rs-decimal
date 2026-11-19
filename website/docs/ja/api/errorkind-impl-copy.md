---
locale: ja
source_revision: 473b8cc7f11457cef7cc926925c4f230754afa71c4dea38c85c3b954c8ca89fd
translation_provenance: en-US-japanese-reviewed
api_id: "ErrorKind::impl Copy"
---
# ErrorKind::impl Copy

Rust の Copy 実装を使います。

```rust
impl Copy for ErrorKind
```

## 引数、戻り値、通常の使い方

Copy はこの小さな列挙型を複製します。借用されたスカラーデータは深くコピーしません。これはネイティブ Rust のトレイト構文で、ブラウザーはメソッドとして公開しません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-impl-copy" locale="ja" />

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
