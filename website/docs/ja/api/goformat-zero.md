---
locale: ja
source_revision: 5fb7d2ba1d450e55c06f4bd243bcd776a0cb6c5e8f2e0f80f16235d6abf720a7
translation_provenance: en-US-japanese-reviewed
api_id: GoFormat.zero
---
# GoFormat.zero

書式フィールド zero を設定します。

```rust
zero: bool
```

## 引数、戻り値、通常の使い方

数値の左側の埋め草を符号の後にゼロで追加します。left が true、または d に明示的な precision がある場合は無視します。既定値は false です。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-zero" locale="ja" />

## ブラウザーの制約

[リソースポリシー](../resource-policy)は書式幅を 65536 バイト、精度を 1000 に制限し、固定小数点の書式では係数増大も確認します。バッファーは 65536 バイトまでです。追記は保守的なサイズ見積もりを使うため、実際にはちょうど収まる操作も拒否する場合があります。リソースエラーは入力やバッファーを変更しません。print は別途長い表示テキストを切り詰めます。全結果を調べるには長さを確認するか、小さな部分に分けて出力してください。

## 関連メソッド

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
