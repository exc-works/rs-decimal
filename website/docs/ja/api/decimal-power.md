---
locale: ja
source_revision: 01c476851989c18cecc0970b361010eee7cb4cfd902d7e4240be18102abc0899
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::power
---
# Decimal::power

元のスケールで整数乗します。

```rust
fn power (& self , power : i64) -> Result < Self , DecimalError >
```

## 引数、戻り値、通常の使い方

power: i64 を bigint で渡します。入力のスケールへ最近接偶数丸めを行い、負の指数では逆数を取ります。2.00 の -3 乗は 0.12 ですが、整数スケールの 2 の -3 乗は 0 です。0 の 0 乗は初期化済みの 1、0 の負の累乗は DivideByZero です。コアの中間上限を超える巨大な指数は割り当て前に Overflow を返します。ブラウザーの中間スケール・係数の上限はさらに小さく、0 と +/-1 ではコアの高速経路を保持します。入力は借用します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-power" locale="ja" />

## 関連メソッド

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
