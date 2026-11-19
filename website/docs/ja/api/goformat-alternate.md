---
locale: ja
source_revision: 711eb90b21b1b79f0303fceb3e98f79a6b9b78a5eb632f30a661f662027c1167
translation_provenance: en-US-japanese-reviewed
api_id: GoFormat.alternate
---
# GoFormat.alternate

書式フィールド alternate を設定します。

```rust
alternate: bool
```

## 引数、戻り値、通常の使い方

f/F/e/E では小数点を保持し、g/G の一般書式では末尾のゼロを保持します。別の基数での表示を意味しません。既定値は false です。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-alternate" locale="ja" />

## 関連メソッド

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
