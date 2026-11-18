---
locale: ja
source_revision: e9b605c9c2b762d25f61c081bee35aa1c375be0cb3cab1e12e2bd0938c7a3c70
translation_provenance: en-US-japanese-reviewed
api_id: GoFormat
---
# GoFormat

フィールドオブジェクトで 1 回の書式設定を指定します。

```rust
pub struct GoFormat
```

## 引数、戻り値、通常の使い方

フィールドは Rust の構造体と同じ verb、width、precision、left、plus、space、zero、alternate です。bridge の GoFormat.default() は Rust の既定値を読み取り、新しい可変オブジェクトを返します。その変更はグローバルな書式設定ではありません。width/precision の None は null、Some は非負の wasm32 整数 number に対応します。Rust トレイト構文はネイティブ専用です。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat" locale="ja" />

## ブラウザーの制約

[リソースポリシー](../resource-policy)は書式幅を 65536 バイト、精度を 1000 に制限し、固定小数点の書式では係数増大も確認します。バッファーは 65536 バイトまでです。追記は保守的なサイズ見積もりを使うため、実際にはちょうど収まる操作も拒否する場合があります。リソースエラーは入力やバッファーを変更しません。print は別途長い表示テキストを切り詰めます。全結果を調べるには長さを確認するか、小さな部分に分けて出力してください。

## 関連メソッド

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
