---
locale: ja
source_revision: 561bf3d1118c3ce8a6c2b94160186e43d5ce2153ae6df6a545c57b9a2538ed41
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::modulo
---
# Decimal::modulo

ゼロ方向に切り捨てる除算の符号付き剰余を返します。

```rust
fn modulo (& self , other : & Self) -> Result < Self , DecimalError >
```

## 引数、戻り値、通常の使い方

other: &Decimal。Rust は Result<Decimal, DecimalError> を返します。剰余は被除数の符号と両オペランドの大きい方のスケールを持ち、非負のユークリッド剰余ではありません。除数がゼロなら DivideByZero です。入力は借用します。商と剰余の両方が必要なら quo_rem を使います。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-modulo" locale="ja" />

## 関連メソッド

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
