---
locale: ja
source_revision: 5f1761508e55494581ac3ef80177b3b3480f53a919c2efbe231b66c02ccf3a14
translation_provenance: en-US-japanese-reviewed
api_id: GoFormat.precision
---
# GoFormat.precision

書式フィールド precision を設定します。

```rust
precision: Option < usize >
```

## 引数、戻り値、通常の使い方

precision の意味は書式指定文字によります。f/F は小数点以下の桁数、e/E は先頭の数字に続く桁数、g/G は有効桁数（0 は 1 と扱う）、d は整数の最小桁数です。None/null では各書式の既定値を使い、f はスケールを保持し、e は 6 桁を使います。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-precision" locale="ja" />

## 関連メソッド

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
