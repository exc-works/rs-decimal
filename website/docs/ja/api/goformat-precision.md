---
locale: ja
source_revision: b5c0056c42f3f4e6d3613af84d12321baa6359b54407d77f1aacdb7d5d883dd7
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

## ブラウザーの制約

[リソースポリシー](../resource-policy)は書式幅を 65536 バイト、精度を 1000 に制限し、固定小数点の書式では係数増大も確認します。バッファーは 65536 バイトまでです。追記は保守的なサイズ見積もりを使うため、実際にはちょうど収まる操作も拒否する場合があります。リソースエラーは入力やバッファーを変更しません。print は別途長い表示テキストを切り詰めます。全結果を調べるには長さを確認するか、小さな部分に分けて出力してください。

## 関連メソッド

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
