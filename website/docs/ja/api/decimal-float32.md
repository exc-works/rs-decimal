---
locale: ja
source_revision: 821c4c23615da2c03d94630c3843c11502e27fc2644faac01dc3a81234b52029
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::float32
---
# Decimal::float32

IEEE binary32 へ直接変換し、正確性を返します。

```rust
fn float32 (& self) -> (f32 , bool)
```

## 引数、戻り値、通常の使い方

引数なし。Rust は (f32, bool) を返し、[number, exact] に対応します。中間値は偶数側へ丸めます。フラグは単なる変換の成功ではなく、小数を正確に表現できることを示します。0.1 は不正確です。オーバーフローは符号付き無限大と false を返し、アンダーフローでは符号付きゼロになる場合があります。binary64 に丸めてからではなく、直接 binary32 に丸めます。例では JSON が特殊値を隠さないように IEEE ビット列を記録します。playground の print も NaN、無限大、負のゼロに、転送された binary64 のビット列を添えて表示します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-float32" locale="ja" />

## ブラウザーの制約

整数の転送は精度を失う JS number を経由しません。変換タプルは Rust の値／フラグの順序を保持します。false は正確な結果を保証しません。借用入力は変わらず、コンストラクターは結果を初期化します。[リソースポリシー](../resource-policy)はライブラリエラーとは別に、整数入力、出力スケール、有理数の係数増大を制限します。Rust のインポート構文はネイティブで検証するものとして明示します。

## 関連メソッド

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
