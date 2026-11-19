---
locale: ja
source_revision: 9b1c2ac4982ef348ecf9982f858e7f552532e21dc0129fa816673741276acaf7
translation_provenance: en-US-japanese-reviewed
api_id: GoFormat.left
---
# GoFormat.left

書式フィールド left を設定します。

```rust
left: bool
```

## 引数、戻り値、通常の使い方

右側を空白で埋めます。左揃えは数値のゼロ埋めに優先します。既定値は false です。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-left" locale="ja" />

## 関連メソッド

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
