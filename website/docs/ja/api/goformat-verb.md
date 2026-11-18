---
locale: ja
source_revision: 349cbd7b8c032b6b9b799b90d2d1981de5ba6641cd94e6d5d2f6cdc9c436f5e2
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

## ブラウザーの制約

[リソースポリシー](../resource-policy)は書式幅を 65536 バイト、精度を 1000 に制限し、固定小数点の書式では係数増大も確認します。バッファーは 65536 バイトまでです。追記は保守的なサイズ見積もりを使うため、実際にはちょうど収まる操作も拒否する場合があります。リソースエラーは入力やバッファーを変更しません。print は別途長い表示テキストを切り詰めます。全結果を調べるには長さを確認するか、小さな部分に分けて出力してください。

## 関連メソッド

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
