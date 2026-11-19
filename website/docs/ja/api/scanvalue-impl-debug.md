---
locale: ja
source_revision: ff9a08f2b3feca78c7b4250dd2c69ee67ba05009b21ca77af28e0824e74ae8d9
translation_provenance: en-US-japanese-reviewed
api_id: "ScanValue::impl Debug"
---
# ScanValue::impl Debug

Rust の Debug 実装を使います。

```rust
impl Debug for ScanValue
```

## 引数、戻り値、通常の使い方

Debug は診断用テキストを提供し、安定した通信シリアライズ形式ではありません。これはネイティブ Rust のトレイト構文で、ブラウザーはメソッドとして公開しません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-impl-debug" locale="ja" />

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
