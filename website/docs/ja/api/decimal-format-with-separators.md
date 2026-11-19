---
locale: ja
source_revision: c3fbe0927c540d91118929ccf7291b490ab739216719c7655f9f81744f8edff3
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::format_with_separators
---
# Decimal::format_with_separators

整数の桁を区切り、小数点の区切り文字を選択します。

```rust
fn format_with_separators (& self , thousands : char , decimal : char) -> String
```

## 引数、戻り値、通常の使い方

thousands: char、decimal: char。整数を 3 桁ずつ区切り、保存した小数点以下の全桁を含む String/string を返します。JS の各引数は補助文字も含め正確に 1 つの Unicode スカラー値である必要があります。単独のサロゲートや複数文字の文字列は拒否します。thousands の NUL は桁区切りを無効にし、decimal の NUL はドットを選びます。区切り文字はリテラルであり、ロケール名ではありません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-decimal-format-with-separators" locale="ja" />

## 関連メソッド

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
