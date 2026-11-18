---
locale: ja
source_revision: 76c1141f3b7421518d2bc3a9f102aa654b735f07d57812e6208ffd1b784510f2
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

## ブラウザーの制約

config と serde 機能は有効です。ソーステキスト／バイト列は UTF-8 で 8192 バイトまでです。Rust は解析前に小数テキストとデコード済み JSON 文字列を確認するため、エスケープで[リソースポリシー](../resource-policy)を回避できません。128 ビット整数には範囲内の bigint が必要です。借用する Decimal/NullDecimal ハンドルはデコード時に使用可能である必要があります。変換は入力を消費せず、独立した値を作ります。汎用 Serde トレイトの動作は具体的な JSON アダプターで示します。

## 関連メソッド

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
