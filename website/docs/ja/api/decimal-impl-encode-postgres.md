---
locale: ja
source_revision: ca5166a5653606ecffcf807579adb23570a7902ad8051649236d87831df9c02a
translation_provenance: en-US-japanese-reviewed
api_id: "Decimal::impl Encode < '_ , Postgres >"
---
# Decimal::impl Encode < '_ , Postgres >

ネイティブの SQLx PostgreSQL NUMERIC 連携を使います。

```rust
impl Encode < '_ , Postgres > for Decimal
```

## 通常の使い方と注意点

postgres 機能と SQLx 0.8 の PostgreSQL 型が必要です。この実装はネイティブ専用です。ブラウザーの playground はデータベース接続を開きません。encode_by_ref(&self, &mut PgArgumentBuffer) は `Result<IsNull, BoxDynError>` を返します。借用は値を保持し、エンコード成功後だけ既存の引数バッファーにバイト列を追記します。未初期化の Decimal でも NULL ではない NUMERIC のゼロにエンコードします。バイナリ形式はビッグエンディアンの基数 10000 のグループを使い、表示スケールを保持し、float/BigDecimal を経由しません。16383 を超えるスケールや 131072 桁を超える整数部分は拒否します。エンコードエラー時は宛先バッファーを保持します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-encode-postgres" locale="ja" />

## 実行

この項目はネイティブ Rust のトレイト構文であり、同名の JavaScript 演算子や SQLx オブジェクトを公開しません。既定のネイティブドキュメントテストでこの例を実行します。

## 関連メソッド

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
