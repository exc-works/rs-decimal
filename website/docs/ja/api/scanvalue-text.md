---
locale: ja
source_revision: 66f48b07b8a5c080eb29b01047825ba3448ab4e979a526ae96e347e482fef177
translation_provenance: en-US-japanese-reviewed
api_id: "ScanValue::Text"
---
# ScanValue::Text

SQL 入力の Text バリアントを選択します。

```rust
Text (& 'a str)
```

## 引数、戻り値、通常の使い方

正確な小数テキストを借用します。対応する外側の二重引用符 1 組を除去しますが、エスケープを JSON としてデコードしません。ゼロ以外の値の末尾のスケールは解析後も保持します。バリアントは scan に借用され、同じブラウザー実行内で再利用できます。偽造した値や別の実行の値は拒否します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-text" locale="ja" />

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
