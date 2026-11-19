---
locale: ja
source_revision: 6a03ad3f2e6f0755e57eb89608d0891566bd0e9c4b875afd8935ad4e0e27dd6d
translation_provenance: en-US-japanese-reviewed
api_id: GoFormat::impl Default::default
---
# GoFormat::impl Default::default

Rust の Default::default 実装を使います。

```rust
fn default () -> Self
```

## 引数、戻り値、通常の使い方

GoFormat は小さな Copy フィールド構造体です。Clone/Copy はオプションを保持し、Debug はフィールドを表示します。Default は v、width/precision は None、すべてのフラグは false を選びます。この Rust 構文はネイティブで検証します。JavaScript のオブジェクトは通常の JS フィールドオブジェクトと GoFormat.default() を使います。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-impl-default-default" locale="ja" />

## 関連メソッド

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
