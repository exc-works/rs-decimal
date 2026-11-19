---
locale: ja
source_revision: 9dbbcca20e9ec70948f275a66820b5aebd2bfa7f33ae4f382b6d22c025c682c3
translation_provenance: en-US-japanese-reviewed
api_id: "DecimalError::impl Clone"
---
# DecimalError::impl Clone

Rust の Clone 実装を使います。

```rust
impl Clone for DecimalError
```

## 引数、戻り値、通常の使い方

Clone は値全体を保持しますが、借用された ScanValue スライスは元のデータを参照し続けます。これはネイティブ Rust のトレイト構文で、ブラウザーはメソッドとして公開しません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-impl-clone" locale="ja" />

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
