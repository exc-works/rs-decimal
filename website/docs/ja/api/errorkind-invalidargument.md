---
locale: ja
source_revision: e14d22e2fbe70e02eda750145ed5b1a70252e171e63aeaafcfc01281be24e818
translation_provenance: en-US-japanese-reviewed
api_id: "ErrorKind::InvalidArgument"
---
# ErrorKind::InvalidArgument

InvalidArgument の失敗を識別します。

```rust
InvalidArgument
```

## 引数、戻り値、通常の使い方

引数、検証規則、メッセージテンプレートが不正です。ErrorKind はメッセージ文字列ではなく、安定した分類です。ブラウザーの列挙値は Rust のバリアント名です。構築したエラーでは matches() を使い、捕捉した bridge 例外では kinds 配列を使います。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-invalidargument" locale="ja" />

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
