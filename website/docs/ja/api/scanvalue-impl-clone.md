---
locale: ja
source_revision: 11ee599a7357841e984cdaecadf4da635302996e14ae33191c323e57e59262ff
translation_provenance: en-US-japanese-reviewed
api_id: "ScanValue::impl Clone"
---
# ScanValue::impl Clone

Rust の Clone 実装を使います。

```rust
impl Clone for ScanValue
```

## 引数、戻り値、通常の使い方

Clone は値全体を保持しますが、借用された ScanValue スライスは元のデータを参照し続けます。これはネイティブ Rust のトレイト構文で、ブラウザーはメソッドとして公開しません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-impl-clone" locale="ja" />

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
