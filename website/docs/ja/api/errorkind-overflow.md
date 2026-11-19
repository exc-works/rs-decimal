---
locale: ja
source_revision: 368adfaeea00a469d274e0e87fc3e84f66ca2848d32cfd8759858b2bac06fe99
translation_provenance: en-US-japanese-reviewed
api_id: "ErrorKind::Overflow"
---
# ErrorKind::Overflow

Overflow の失敗を識別します。

```rust
Overflow
```

## 引数、戻り値、通常の使い方

係数、スケール、操作がコアの対応範囲を超えています。ErrorKind はメッセージ文字列ではなく、安定した分類です。ブラウザーの列挙値は Rust のバリアント名です。構築したエラーでは matches() を使い、捕捉した bridge 例外では kinds 配列を使います。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-overflow" locale="ja" />

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
