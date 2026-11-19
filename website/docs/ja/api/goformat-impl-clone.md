---
locale: ja
source_revision: 88974eef4784bf8f1c8b6ec4ce25d3380a2169d449467ecdb4cee7997d0acdd8
translation_provenance: en-US-japanese-reviewed
api_id: GoFormat::impl Clone
---
# GoFormat::impl Clone

Rust の Clone 実装を使います。

```rust
impl Clone for GoFormat
```

## 引数、戻り値、通常の使い方

GoFormat は小さな Copy フィールド構造体です。Clone/Copy はオプションを保持し、Debug はフィールドを表示します。Default は v、width/precision は None、すべてのフラグは false を選びます。この Rust 構文はネイティブで検証します。JavaScript のオブジェクトは通常の JS フィールドオブジェクトと GoFormat.default() を使います。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-impl-clone" locale="ja" />

## 関連メソッド

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
