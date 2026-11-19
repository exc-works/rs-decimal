---
locale: ja
source_revision: c5bc8b97bab18c557c0918fb7ad15cabc2fd824c618c469b8f007d1f6299272b
translation_provenance: en-US-japanese-reviewed
api_id: "ErrorKind::DivideByZero"
---
# ErrorKind::DivideByZero

DivideByZero の失敗を識別します。

```rust
DivideByZero
```

## 引数、戻り値、通常の使い方

除数がゼロです。ErrorKind はメッセージ文字列ではなく、安定した分類です。ブラウザーの列挙値は Rust のバリアント名です。構築したエラーでは matches() を使い、捕捉した bridge 例外では kinds 配列を使います。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-dividebyzero" locale="ja" />

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
