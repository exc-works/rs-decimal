---
locale: ja
source_revision: 17e780c5fcd60ccd7701083f059fdd79ff3607aa9dff1b3dfa127b396f366781
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::new_from_float32
---
# Decimal::new_from_float32

浮動小数点へ戻すと同じ値になる、最短の十進表現から構築します。

```rust
fn new_from_float32 (value : f32) -> Result < Self , DecimalError >
```

## 引数、戻り値、通常の使い方

value: f32 を JS number で渡します。bridge はまず binary64 を binary32 に狭めるため、16777217 は 16777216 になります。二進浮動小数点ですでに失われた小数の桁は復元できません。正確な小数入力には new_from_string を使ってください。NaN と正負の無限大は InvalidFormat です。有限の f64 でも f32 への変換でオーバーフローすれば InvalidFormat です。負のゼロは符号のない初期化済み Decimal ゼロになります。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-new-from-float32" locale="ja" />

## ブラウザーの制約

整数の転送は精度を失う JS number を経由しません。変換タプルは Rust の値／フラグの順序を保持します。false は正確な結果を保証しません。借用入力は変わらず、コンストラクターは結果を初期化します。[リソースポリシー](../resource-policy)はライブラリエラーとは別に、整数入力、出力スケール、有理数の係数増大を制限します。Rust のインポート構文はネイティブで検証するものとして明示します。

## 関連メソッド

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
