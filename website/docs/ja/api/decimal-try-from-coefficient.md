---
locale: ja
source_revision: 3430467a0f3b43857ec837a38512411503e4016266c068e3065f8bd60687547d
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::try_from_coefficient
---
# Decimal::try_from_coefficient

所有する係数とスケールから初期化済みの値を構築します。

```rust
fn try_from_coefficient (coefficient : BigInt , scale : usize) -> Result < Self , DecimalError >
```

## 引数、戻り値、通常の使い方

coefficient: BigInt、scale: usize（非負の小数点以下の桁数）。末尾のゼロと、値がゼロのときのスケールを保持し、テキスト解析の正規化は使いません。isize::MAX を超えると Rust は Overflow を返します。ブラウザーは正確な bigint と wasm32 usize 範囲内の整数 number を使います。スケールが i32::MAX を超えると Overflow を保ち、それ以外で 1000 を超えるスケールは playground のリソース上限を超えます。JS bigint は不変なので、ムーブされるラッパーではありません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-try-from-coefficient" locale="ja" />

## 関連メソッド

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
