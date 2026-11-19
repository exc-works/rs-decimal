---
locale: ja
source_revision: 25693aaef850a3e88dcc638a63de0aa32a65fe1057c9bac969c2400862072704
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::string
---
# Decimal::string

固定小数点のテキストに書式設定します。

```rust
fn string (& self) -> String
```

## 引数、戻り値、通常の使い方

引数なし。科学表記を使わず String/string を返します。小数部分の不要な末尾のゼロを除去し、整数のゼロは保持します。ドットを使い、桁区切りは行いません。負のゼロは別の表現を持ちません。既定のゼロは初期化せずに 0 と出力します。書式設定は入力を借用します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-decimal-string" locale="ja" />

## 関連メソッド

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
