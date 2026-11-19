---
locale: ja
source_revision: 0989f60fe84cf7be4e7708a7140f9cc16468c21ad57d855b67f896084ddd8606
translation_provenance: en-US-japanese-reviewed
api_id: GoFormat.plus
---
# GoFormat.plus

書式フィールド plus を設定します。

```rust
plus: bool
```

## 引数、戻り値、通常の使い方

非負の数値出力に + を付けます。space に優先します。数値の書式指定文字に適用し、v/s/q には適用しません。負の値は - を保持します。既定値は false です。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-plus" locale="ja" />

## 関連メソッド

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
