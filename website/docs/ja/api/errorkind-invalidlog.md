---
locale: ja
source_revision: c456812ef50493713a2bbd81d6c57db5a0f730099b5878e83b3d4f358245d822
translation_provenance: en-US-japanese-reviewed
api_id: "ErrorKind::InvalidLog"
---
# ErrorKind::InvalidLog

InvalidLog の失敗を識別します。

```rust
InvalidLog
```

## 引数、戻り値、通常の使い方

対数の入力が非正です。ErrorKind はメッセージ文字列ではなく、安定した分類です。ブラウザーの列挙値は Rust のバリアント名です。構築したエラーでは matches() を使い、捕捉した bridge 例外では kinds 配列を使います。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-invalidlog" locale="ja" />

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
