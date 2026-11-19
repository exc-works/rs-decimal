---
locale: ja
source_revision: fe33f9fd8631eeeae5a499a47abbc262340fc190757785abe0749a14c97bf791
translation_provenance: en-US-japanese-reviewed
api_id: GoFormat.width
---
# GoFormat.width

書式フィールド width を設定します。

```rust
width: Option < usize >
```

## 引数、戻り値、通常の使い方

最小幅は符号と小数点を含みます。長いテキストを切り詰めず、足りない分を埋めます。None/null は最小幅を無効にします。コアの書式設定では出力バイト数を数えます。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-width" locale="ja" />

## 関連メソッド

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
