---
locale: ja
source_revision: 7fadd186172ed1d2675ccc16ae4020af71fd55a578a479ca70e021285e76f370
translation_provenance: en-US-japanese-reviewed
api_id: "Decimal::impl From < BigInt >"
---
# Decimal::impl From < BigInt >

Rust の From で BigInt を正確に変換します。

```rust
impl From < BigInt > for Decimal
```

## 通常の使い方と注意点

BigInt を消費し、スケール 0 の初期化済み Decimal を返します。浮動小数点や丸めを使わず、元の整数型からの変換は失敗しません。Decimal::default() とは異なり、ゼロも初期化済みです。BigInt の所有権は小数へムーブされます。ブラウザーでは明示的な名前付きコンストラクターか ConfigValue の signed/unsigned バリアントを使います。Rust の From/Into 構文はネイティブ専用です。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-from-bigint" locale="ja" />

## 実行

この項目はネイティブ Rust のトレイト構文であり、同名の JavaScript 演算子や SQLx オブジェクトを公開しません。既定のネイティブドキュメントテストでこの例を実行します。

## 関連メソッド

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
