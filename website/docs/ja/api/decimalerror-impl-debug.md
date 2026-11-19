---
locale: ja
source_revision: f91df84f154c9cc1b9dd7b8a5909be0f1702eb57d28ff7a024367e2c25d73638
translation_provenance: en-US-japanese-reviewed
api_id: "DecimalError::impl Debug"
---
# DecimalError::impl Debug

Rust の Debug 実装を使います。

```rust
impl Debug for DecimalError
```

## 引数、戻り値、通常の使い方

Debug は診断用テキストを提供し、安定した通信シリアライズ形式ではありません。これはネイティブ Rust のトレイト構文で、ブラウザーはメソッドとして公開しません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-impl-debug" locale="ja" />

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
