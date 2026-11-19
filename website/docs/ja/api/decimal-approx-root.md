---
locale: ja
source_revision: 2e472bd0598b8ec796fb5adb09d98ce669a8d1defb11e5244f124c816e8d2cd8
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::approx_root
---
# Decimal::approx_root

正の次数の累乗根を計算します。Rust は Result<Decimal, DecimalError>、bridge は Decimal または構造化された例外を返します。degree: i64 を bigint で渡します。次数は正である必要があり、それ以外は InvalidRoot です。奇数次は負の入力を許可し、負の値の偶数次根は NegativeRoot です。次数 1 は最近接偶数丸めによるスケール変更です。既定精度は max(入力の小数点以下の桁数, 20) であり、有効数字 20 桁ではありません。丸めは最近接偶数丸めです。精度 0 の sqrt(6.25) のように正確に中間になる根は 2 に丸めます。approx_root という名前でも、Rust は整数比較で丸めを証明します。コアの計算量や反復上限に達すると NoConvergence を返します。入力は借用して変更せず、既定のゼロを含む結果は初期化済みです。

```rust
fn approx_root (& self , degree : i64) -> Result < Self , DecimalError >
```

## 通常の使い方

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="roots-decimal-approx-root" locale="ja" />

## 関連メソッド

[sqrt](./decimal-sqrt) · [approx_root_with_prec](./decimal-approx-root-with-prec) · [power](./decimal-power)
