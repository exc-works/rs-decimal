---
locale: ja
source_revision: f89bca6a9b41c5dfaa6d534efc134c1f42c007ba065d09915785753b5a161115
translation_provenance: en-US-japanese-reviewed
api_id: GoFormat
---
# GoFormat

フィールドオブジェクトで 1 回の書式設定を指定します。

```rust
pub struct GoFormat
```

## 引数、戻り値、通常の使い方

フィールドは Rust の構造体と同じ verb、width、precision、left、plus、space、zero、alternate です。bridge の GoFormat.default() は Rust の既定値を読み取り、新しい可変オブジェクトを返します。その変更はグローバルな書式設定ではありません。width/precision の None は null、Some は非負の wasm32 整数 number に対応します。Rust トレイト構文はネイティブ専用です。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat" locale="ja" />

## 関連メソッド

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
