---
locale: ja
source_revision: 68b84662c55e7239d15b1fe54a5a2076eb45cea41916719d90f7dbe8ab9e7150
translation_provenance: en-US-japanese-reviewed
api_id: "ErrorKind::InvalidPrecision"
---
# ErrorKind::InvalidPrecision

InvalidPrecision の失敗を識別します。

```rust
InvalidPrecision
```

## 引数、戻り値、通常の使い方

要求した精度が不正です。ErrorKind はメッセージ文字列ではなく、安定した分類です。ブラウザーの列挙値は Rust のバリアント名です。構築したエラーでは matches() を使い、捕捉した bridge 例外では kinds 配列を使います。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-invalidprecision" locale="ja" />

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
