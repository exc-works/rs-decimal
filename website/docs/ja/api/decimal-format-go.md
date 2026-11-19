---
locale: ja
source_revision: f9b1eaa0c6a10e9f32908baa1f2c09a9aafeb3265bffa095bb0a0b210e1e1f05
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::format_go
---
# Decimal::format_go

Go 互換の書式指定文字 1 つと書式オプションを適用します。

```rust
fn format_go (& self , f : GoFormat) -> Result < String , DecimalError >
```

## 引数、戻り値、通常の使い方

f: GoFormat。Result<String, DecimalError>/string を返します。%.2f のような printf パターンではなく、書式指定文字 1 つを受け取ります。対応する文字は v/s/q/d/f/F/e/E/g/G/b です。最近接偶数丸めを使います。未対応の文字や整数でない d は例外ではなく、%! マーカーをテキストとして出力します。ネイティブの符号付き範囲を超える固定精度は InvalidPrecision です。ブラウザーの幅・精度・割り当て制限は別に適用します。省略した JS フィールドは Rust の既定値を使い、不明なフィールドと不正な型は拒否します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-decimal-format-go" locale="ja" />

## 関連メソッド

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
