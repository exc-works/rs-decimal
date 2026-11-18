---
locale: ja
source_revision: c07644d3e3d75d66141539a54d5f6c78026aea2ec5a96a31f2d985d0d9ec6fc0
translation_provenance: en-US-japanese-reviewed
api_id: GoFormat::impl Copy
---
# GoFormat::impl Copy

Rust の Copy 実装を使います。

```rust
impl Copy for GoFormat
```

## 引数、戻り値、通常の使い方

GoFormat は小さな Copy フィールド構造体です。Clone/Copy はオプションを保持し、Debug はフィールドを表示します。Default は v、width/precision は None、すべてのフラグは false を選びます。この Rust 構文はネイティブで検証します。bridge のオブジェクトは通常の JS フィールドオブジェクトと GoFormat.default() を使います。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-impl-copy" locale="ja" />

## ブラウザーの制約

[リソースポリシー](../resource-policy)は書式幅を 65536 バイト、精度を 1000 に制限し、固定小数点の書式では係数増大も確認します。バッファーは 65536 バイトまでです。追記は保守的なサイズ見積もりを使うため、実際にはちょうど収まる操作も拒否する場合があります。リソースエラーは入力やバッファーを変更しません。print は別途長い表示テキストを切り詰めます。全結果を調べるには長さを確認するか、小さな部分に分けて出力してください。

## 関連メソッド

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
