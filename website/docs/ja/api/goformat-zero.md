---
locale: ja
source_revision: f893b87d46c7c5082ca48cb1eae39734e63ca91a57f2f944773172a9baf78fca
translation_provenance: en-US-japanese-reviewed
api_id: GoFormat.zero
---
# GoFormat.zero

書式フィールド zero を設定します。

```rust
zero: bool
```

## 引数、戻り値、通常の使い方

数値の左側の埋め草を符号の後にゼロで追加します。left が true、または d に明示的な precision がある場合は無視します。既定値は false です。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-zero" locale="ja" />

## 関連メソッド

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
