---
locale: ja
source_revision: 5acf1a9800c0d02bd1cb1fa68c2c1f7068c1bb36cf20034e0a1e404758ce2bc4
translation_provenance: en-US-japanese-reviewed
api_id: "ErrorKind::InvalidRoot"
---
# ErrorKind::InvalidRoot

InvalidRoot の失敗を識別します。

```rust
InvalidRoot
```

## 引数、戻り値、通常の使い方

累乗根の次数が非正です。ErrorKind はメッセージ文字列ではなく、安定した分類です。ブラウザーの列挙値は Rust のバリアント名です。構築したエラーでは matches() を使い、捕捉した bridge 例外では kinds 配列を使います。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-invalidroot" locale="ja" />

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
