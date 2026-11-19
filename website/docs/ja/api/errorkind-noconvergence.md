---
locale: ja
source_revision: aeb152b6bffb5f406cc2fbe512dd2449186121fa1cd03efbf5d74f0d3891aed1
translation_provenance: en-US-japanese-reviewed
api_id: "ErrorKind::NoConvergence"
---
# ErrorKind::NoConvergence

NoConvergence の失敗を識別します。

```rust
NoConvergence
```

## 引数、戻り値、通常の使い方

上限付きのアルゴリズムが丸め結果を証明できませんでした。数学的な結果が存在しないという意味ではありません。ErrorKind はメッセージ文字列ではなく、安定した分類です。ブラウザーの列挙値は Rust のバリアント名です。構築したエラーでは matches() を使い、捕捉した bridge 例外では kinds 配列を使います。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-noconvergence" locale="ja" />

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
