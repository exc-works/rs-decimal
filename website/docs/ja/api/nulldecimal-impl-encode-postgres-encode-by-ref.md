---
locale: ja
source_revision: f24ab15a12ac3cc5edffa3bb6354648be7a4944a94929ac82037f54fe89c586d
translation_provenance: en-US-japanese-reviewed
api_id: "NullDecimal::impl Encode < '_ , Postgres >::encode_by_ref"
---
# NullDecimal::impl Encode < '_ , Postgres >::encode_by_ref

ネイティブの SQLx PostgreSQL NUMERIC 連携を使います。

```rust
fn encode_by_ref (& self , buf : & mut PgArgumentBuffer) -> Result < IsNull , BoxDynError >
```

## 通常の使い方と注意点

postgres 機能と SQLx 0.8 の PostgreSQL 型が必要です。この実装はネイティブ専用です。ブラウザーの playground はデータベース接続を開きません。encode_by_ref(&self, &mut PgArgumentBuffer) は `Result<IsNull, BoxDynError>` を返します。借用は値を保持し、エンコード成功後だけ既存の引数バッファーにバイト列を追記します。無効な nullable はバイト列なしで IsNull::Yes を返します。有効な値は内部 Decimal をエンコードし、そのゼロが未初期化でも同様です。バイナリ形式はビッグエンディアンの基数 10000 のグループを使い、表示スケールを保持し、float/BigDecimal を経由しません。16383 を超えるスケールや 131072 桁を超える整数部分は拒否します。エンコードエラー時は宛先バッファーを保持します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-nulldecimal-impl-encode-postgres-encode-by-ref" locale="ja" />

## 実行

この項目はネイティブ Rust のトレイト構文であり、同名の JavaScript 演算子や SQLx オブジェクトを公開しません。既定のネイティブドキュメントテストでこの例を実行します。

## 関連メソッド

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
