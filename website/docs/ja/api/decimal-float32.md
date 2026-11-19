---
locale: ja
source_revision: c7bf1c667592f8239154dc21841d27e0f4e94230da52fdab553a96aeaa5e48bf
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

## 関連メソッド

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
