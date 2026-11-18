---
locale: ja
source_revision: 9abf8678e26851a7c2e2283fba52cbf7f426bb86feeae64e6b8312a6a7254ba7
translation_provenance: en-US-japanese-reviewed
api_id: "config::decode_decimal"
---
# config::decode_decimal

明示的に型を指定した設定値をデコードします。

```rust
fn decode_decimal (value : ConfigValue < '_ >) -> Result < Decimal , DecimalError >
```

## 引数、戻り値、通常の使い方

value: ConfigValue。`Result<Decimal, DecimalError>`/新しい Decimal ハンドルを返します。Rust は config を必要とします。非 nullable のデコードはゼロを補うのではなく、Null と無効な Nullable を拒否します。既存の Decimal と有効な Nullable は未初期化のゼロも含め、表現全体を複製します。Text/Number/Bytes は SQL の引用符を除去せず、正確な小数テキストを解析します。浮動小数点にはすでに二進の丸めが含まれます。元の桁を正確に保つにはテキストか bigint を使います。ブラウザーのバリアントは 1 回の実行に属し、デコードまでハンドル／バッファーを借用します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-decode-decimal" locale="ja" />

## ブラウザーの制約

config と serde 機能は有効です。ソーステキスト／バイト列は UTF-8 で 8192 バイトまでです。Rust は解析前に小数テキストとデコード済み JSON 文字列を確認するため、エスケープで[リソースポリシー](../resource-policy)を回避できません。128 ビット整数には範囲内の bigint が必要です。借用する Decimal/NullDecimal ハンドルはデコード時に使用可能である必要があります。変換は入力を消費せず、独立した値を作ります。汎用 Serde トレイトの動作は具体的な JSON アダプターで示します。

## 関連メソッド

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
