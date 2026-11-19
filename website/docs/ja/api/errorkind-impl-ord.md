---
locale: ja
source_revision: 77c1d26d73dd77cab1d0c87534f9e5ec0b5d57b39f76087cd3ef2b288f8c6fae
translation_provenance: en-US-japanese-reviewed
api_id: "ErrorKind::impl Ord"
---
# ErrorKind::impl Ord

Rust の Ord 実装を使います。

```rust
impl Ord for ErrorKind
```

## 引数、戻り値、通常の使い方

ErrorKind を重大度や名前のアルファベット順ではなく、Rust の宣言順に並べます。これはネイティブ Rust のトレイト構文で、ブラウザーはメソッドとして公開しません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-impl-ord" locale="ja" />

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
