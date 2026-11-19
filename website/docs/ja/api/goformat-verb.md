---
locale: ja
source_revision: 0c3a3cc5d3969cdd2cb971de3686daa945e18486f55f8c99179deb863727c501
translation_provenance: en-US-japanese-reviewed
api_id: GoFormat.verb
---
# GoFormat.verb

書式フィールド verb を設定します。

```rust
verb: char
```

## 引数、戻り値、通常の使い方

Unicode スカラー値 1 つを書式指定文字として選びます。既定の v は簡潔な固定小数点テキスト、e/E は科学表記、q は引用符付き、b は二進の係数と p-scale を出力します（IEEE 浮動小数点ではありません）。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-verb" locale="ja" />

## 関連メソッド

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
