---
locale: ja
source_revision: c5619d120e2ab677bb25ae58cef9205a1160d82cd87b1081f2a6532750a83303
translation_provenance: en-US-japanese-reviewed
api_id: GoFormat.space
---
# GoFormat.space

書式フィールド space を設定します。

```rust
space: bool
```

## 引数、戻り値、通常の使い方

plus が false の場合、非負の数値出力の先頭に空白を付けます。負の値は符号を保持します。既定値は false です。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-space" locale="ja" />

## 関連メソッド

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
