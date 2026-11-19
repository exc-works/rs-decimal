---
locale: ja
source_revision: 1b410a44c40ddf1ed7ab08ecafd94bf955a8c6e8ace55716beba7530f0db9122
translation_provenance: en-US-japanese-reviewed
api_id: "ErrorKind::impl Debug"
---
# ErrorKind::impl Debug

Rust の Debug 実装を使います。

```rust
impl Debug for ErrorKind
```

## 引数、戻り値、通常の使い方

Debug は診断用テキストを提供し、安定した通信シリアライズ形式ではありません。これはネイティブ Rust のトレイト構文で、ブラウザーはメソッドとして公開しません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-impl-debug" locale="ja" />

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
