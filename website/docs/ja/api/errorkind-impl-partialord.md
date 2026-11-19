---
locale: ja
source_revision: b6a1d6732343e8fec8b5e4584b880016ecb9ffdc8cf16c927c04637de618fc19
translation_provenance: en-US-japanese-reviewed
api_id: "ErrorKind::impl PartialOrd"
---
# ErrorKind::impl PartialOrd

Rust の PartialOrd 実装を使います。

```rust
impl PartialOrd for ErrorKind
```

## 引数、戻り値、通常の使い方

ErrorKind を重大度や名前のアルファベット順ではなく、Rust の宣言順に並べます。これはネイティブ Rust のトレイト構文で、ブラウザーはメソッドとして公開しません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-impl-partialord" locale="ja" />

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
