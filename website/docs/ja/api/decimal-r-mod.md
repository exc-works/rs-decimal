---
locale: ja
source_revision: 68a6c601d819a031d04a43cce835c2569811cfe3ffa0049911e2723c800f6029
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::r#mod
---
# Decimal::r#mod

ゼロ方向に切り捨てる除算の符号付き剰余を返します。

```rust
fn r#mod (& self , other : & Self) -> Result < Self , DecimalError >
```

## 引数、戻り値、通常の使い方

other: &Decimal。Rust は Result<Decimal, DecimalError> を返します。剰余は被除数の符号と両オペランドの大きい方のスケールを持ち、非負のユークリッド剰余ではありません。除数がゼロなら DivideByZero です。入力は借用します。mod は予約語なので Rust では r#mod、JS では .mod(other) を使います。どちらも同じコアの動作を呼び出します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-r-mod" locale="ja" />

## 関連メソッド

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
