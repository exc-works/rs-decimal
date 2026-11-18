---
locale: ja
source_revision: c98261b50ba3febe344cea19fd963c1739045fe7ff1872b278b97c36e7fef20b
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

## ブラウザーの制約

[リソースポリシー](../resource-policy)は書式幅を 65536 バイト、精度を 1000 に制限し、固定小数点の書式では係数増大も確認します。バッファーは 65536 バイトまでです。追記は保守的なサイズ見積もりを使うため、実際にはちょうど収まる操作も拒否する場合があります。リソースエラーは入力やバッファーを変更しません。print は別途長い表示テキストを切り詰めます。全結果を調べるには長さを確認するか、小さな部分に分けて出力してください。

## 関連メソッド

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
