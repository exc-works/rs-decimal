---
locale: ja
source_revision: f320c1a30df05777794b5d83edd6494fed5ebfe3f6b80e1d48ebc83fa5e17748
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::string_with_trailing_zeros
---
# Decimal::string_with_trailing_zeros

固定小数点のテキストに書式設定します。

```rust
fn string_with_trailing_zeros (& self) -> String
```

## 引数、戻り値、通常の使い方

引数なし。科学表記を使わず String/string を返します。末尾のゼロを含む、保存された小数点以下の全桁を保持します。ドットを使い、桁区切りは行いません。負のゼロは別の表現を持ちません。既定のゼロは初期化せずに 0 と出力します。書式設定は入力を借用します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-decimal-string-with-trailing-zeros" locale="ja" />

## 関連メソッド

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
