---
locale: ja
source_revision: e2bc3815435c7e993973dc27c3f32515fd48346d012f56633ed09feff4f0aa99
translation_provenance: en-US-japanese-reviewed
api_id: "ErrorKind::NegativeRoot"
---
# ErrorKind::NegativeRoot

NegativeRoot の失敗を識別します。

```rust
NegativeRoot
```

## 引数、戻り値、通常の使い方

負の値に偶数次根を要求しています。ErrorKind はメッセージ文字列ではなく、安定した分類です。ブラウザーの列挙値は Rust のバリアント名です。構築したエラーでは matches() を使い、捕捉した bridge 例外では kinds 配列を使います。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-negativeroot" locale="ja" />

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
