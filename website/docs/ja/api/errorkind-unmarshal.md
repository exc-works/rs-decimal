---
locale: ja
source_revision: 274359ec57012f0ef20672a2e83c98afd2d82df0c75391b519022c4925e09ca8
translation_provenance: en-US-japanese-reviewed
api_id: "ErrorKind::Unmarshal"
---
# ErrorKind::Unmarshal

Unmarshal の失敗を識別します。

```rust
Unmarshal
```

## 引数、戻り値、通常の使い方

デコードに失敗しました。InvalidFormat など別の種類が原因を表す場合があるため、1 種類だけと仮定せず包含を調べてください。ErrorKind はメッセージ文字列ではなく、安定した分類です。ブラウザーの列挙値は Rust のバリアント名です。構築したエラーでは matches() を使い、捕捉した bridge 例外では kinds 配列を使います。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-unmarshal" locale="ja" />

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
