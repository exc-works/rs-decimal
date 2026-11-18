---
locale: ja
source_revision: 84f61035965dcc9b605c00e7c877afbafcb7870fb2b3b89e0902348c80e03dd7
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

## ブラウザーの制約

[リソースポリシー](../resource-policy)は書式幅を 65536 バイト、精度を 1000 に制限し、固定小数点の書式では係数増大も確認します。バッファーは 65536 バイトまでです。追記は保守的なサイズ見積もりを使うため、実際にはちょうど収まる操作も拒否する場合があります。リソースエラーは入力やバッファーを変更しません。print は別途長い表示テキストを切り詰めます。全結果を調べるには長さを確認するか、小さな部分に分けて出力してください。

## 関連メソッド

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
