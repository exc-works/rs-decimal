---
locale: ja
source_revision: b4b7e965750a7655fdeb57385feedb3c62874c4ab66fdfb6a3f2062d579960d6
translation_provenance: en-US-japanese-reviewed
api_id: "ErrorKind::impl Clone"
---
# ErrorKind::impl Clone

Rust の Clone 実装を使います。

```rust
impl Clone for ErrorKind
```

## 引数、戻り値、通常の使い方

Clone は値全体を保持しますが、借用された ScanValue スライスは元のデータを参照し続けます。これはネイティブ Rust のトレイト構文で、ブラウザーはメソッドとして公開しません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-impl-clone" locale="ja" />

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
