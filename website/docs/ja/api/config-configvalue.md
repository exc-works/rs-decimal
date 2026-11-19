---
locale: ja
source_revision: fe8961def1f2d55b79394ddbdd152dccfbb21eaad02a1ae36eaa6898f24726be
translation_provenance: en-US-japanese-reviewed
api_id: "config::ConfigValue"
---
# config::ConfigValue

明示的に型を指定した設定値をデコードします。

```rust
pub enum ConfigValue< 'a >
```

## 引数、戻り値、通常の使い方

型付き設定アダプターはテキストとそのバイト列、正確な数値テキスト、符号付き／符号なし 128 ビット整数、浮動小数点、借用 Decimal/NullDecimal を扱います。Rust は config を必要とします。非 nullable のデコードはゼロを補うのではなく、Null と無効な Nullable を拒否します。既存の Decimal と有効な Nullable は未初期化のゼロも含め、表現全体を複製します。Text/Number/Bytes は SQL の引用符を除去せず、正確な小数テキストを解析します。浮動小数点にはすでに二進の丸めが含まれます。元の桁を正確に保つにはテキストか bigint を使います。ブラウザーのバリアントは 1 回の実行に属し、デコードまでハンドル／バッファーを借用します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue" locale="ja" />

## 関連メソッド

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
