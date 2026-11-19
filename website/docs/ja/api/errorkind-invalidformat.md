---
locale: ja
source_revision: 72523519c8ccf1aad4c8baebb6ac1ef42e8070b9cc7f7f72b600ff77781eaf4e
translation_provenance: en-US-japanese-reviewed
api_id: "ErrorKind::InvalidFormat"
---
# ErrorKind::InvalidFormat

InvalidFormat の失敗を識別します。

```rust
InvalidFormat
```

## 引数、戻り値、通常の使い方

入力が対応する有限の小数表現ではありません。ErrorKind はメッセージ文字列ではなく、安定した分類です。ブラウザーの列挙値は Rust のバリアント名です。構築したエラーでは matches() を使い、捕捉した bridge 例外では kinds 配列を使います。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-invalidformat" locale="ja" />

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
