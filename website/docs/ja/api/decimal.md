---
locale: ja
source_revision: 56094af9f622985dec933077af65cd0cc673023dc5a3a4be3553f2af47b621d4
translation_provenance: en-US-japanese-reviewed
api_id: Decimal
---
# Decimal

正確な係数と非負の小数スケールを保存します。

```rust
pub struct Decimal
```

## 通常の使い方と意味

数値は係数 × 10^-scale です。スケールは小数点以下の桁数で、保持した末尾のゼロも含みます。既定値は未初期化の数値ゼロです。コンストラクターは初期化済みの値を作り、借用メソッドは入力を保持し、消費するメソッドは入力のブラウザーラッパーを無効にします。JavaScript では名前付きコンストラクターを使ってください。new Decimal() は公開コンストラクターではありません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal" locale="ja" />

共通の例をネイティブ Rust とブラウザーで実行します。

## 関連ドキュメント

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
