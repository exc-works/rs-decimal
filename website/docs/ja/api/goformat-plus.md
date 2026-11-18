---
locale: ja
source_revision: b8d0350ab2d3257ebc2822de61360d2033648c92799436b97481a4c0e75947bf
translation_provenance: en-US-japanese-reviewed
api_id: GoFormat.plus
---
# GoFormat.plus

書式フィールド plus を設定します。

```rust
plus: bool
```

## 引数、戻り値、通常の使い方

非負の数値出力に + を付けます。space に優先します。数値の書式指定文字に適用し、v/s/q には適用しません。負の値は - を保持します。既定値は false です。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-plus" locale="ja" />

## ブラウザーの制約

[リソースポリシー](../resource-policy)は書式幅を 65536 バイト、精度を 1000 に制限し、固定小数点の書式では係数増大も確認します。バッファーは 65536 バイトまでです。追記は保守的なサイズ見積もりを使うため、実際にはちょうど収まる操作も拒否する場合があります。リソースエラーは入力やバッファーを変更しません。print は別途長い表示テキストを切り詰めます。全結果を調べるには長さを確認するか、小さな部分に分けて出力してください。

## 関連メソッド

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
