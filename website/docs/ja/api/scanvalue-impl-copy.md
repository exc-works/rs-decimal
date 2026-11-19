---
locale: ja
source_revision: 7eeabc8ee1ace4f862b185d7c83f7d0a53594b9ff67ac15d988dde4ffaa12e4b
translation_provenance: en-US-japanese-reviewed
api_id: "ScanValue::impl Copy"
---
# ScanValue::impl Copy

Rust の Copy 実装を使います。

```rust
impl Copy for ScanValue
```

## 引数、戻り値、通常の使い方

Copy はこの小さな列挙型を複製します。借用されたスカラーデータは深くコピーしません。これはネイティブ Rust のトレイト構文で、ブラウザーはメソッドとして公開しません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-impl-copy" locale="ja" />

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
