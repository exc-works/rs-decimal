---
locale: ja
source_revision: 4beaa8ccdf2751ba7bce74d134b8de29fabd5c0d82d0d4eebee1575a204be12d
translation_provenance: en-US-japanese-reviewed
api_id: reexport::GoFormat
---
# reexport::GoFormat

クレートルートから GoFormat をインポートします。

```rust
pub use formatting::GoFormat as GoFormat
```

## 引数、戻り値、通常の使い方

これは公開 Rust 構造体の再エクスポートであり、別の実装ではありません。ブラウザーはフィールドオブジェクトのアダプターと、Rust に基づく既定値ファクトリーを提供します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-reexport-goformat" locale="ja" />

## ブラウザーの制約

[リソースポリシー](../resource-policy)は書式幅を 65536 バイト、精度を 1000 に制限し、固定小数点の書式では係数増大も確認します。バッファーは 65536 バイトまでです。追記は保守的なサイズ見積もりを使うため、実際にはちょうど収まる操作も拒否する場合があります。リソースエラーは入力やバッファーを変更しません。print は別途長い表示テキストを切り詰めます。全結果を調べるには長さを確認するか、小さな部分に分けて出力してください。

## 関連メソッド

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
