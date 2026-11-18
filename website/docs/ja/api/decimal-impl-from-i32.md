---
locale: ja
source_revision: e8a63fcc23b1c2f1d265bc550f348b1be57084355aeb3ae571fd3b961cb35120
translation_provenance: en-US-japanese-reviewed
api_id: "Decimal::impl From < i32 >"
---
# Decimal::impl From < i32 >

Rust の From で i32 を正確に変換します。

```rust
impl From<i32> for Decimal
```

## 通常の使い方と注意点

i32 を消費し、スケール 0 の初期化済み Decimal を返します。浮動小数点や丸めを使わず、元の整数型からの変換は失敗しません。Decimal::default() とは異なり、ゼロも初期化済みです。元の範囲は Rust 整数型の範囲です。isize/usize の幅はネイティブターゲットに依存します。ブラウザーでは明示的な名前付きコンストラクターか ConfigValue の signed/unsigned バリアントを使います。Rust の From/Into 構文はネイティブ専用です。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-from-i32" locale="ja" />

## 実行

この項目はネイティブ Rust のトレイト構文であり、同名の JavaScript 演算子や SQLx オブジェクトを公開しません。既定のネイティブドキュメントテストでこの例を実行します。

## 関連メソッド

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
