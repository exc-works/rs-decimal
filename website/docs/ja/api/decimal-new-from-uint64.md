---
locale: ja
source_revision: a35340d9c55b8a08723d8ff0cfa4529bad65cad374d1045d7ce321047fc2a260
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::new_from_uint64
---
# Decimal::new_from_uint64

符号なし 64 ビット係数から小数を作成します。非負の整数入力に対して、小数点以下の桁数を別に指定するときに使います。

```rust
fn new_from_uint64 (value : u64 , prec : isize) -> Result < Self , DecimalError >
```

## 引数と戻り値

`value: u64` → 非負の `bigint`、`prec: isize` → 整数の `number`。`value × 10^-prec` を返します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-from-uint64" locale="ja" />

## 使用上の注意と制約

負の値と u64::MAX を超える値は Rust に入る前に拒否されます。負の精度はライブラリエラー `InvalidPrecision` です。JavaScript number では u64 の全範囲を安全に保持できません。

ブラウザーは[リソースポリシー](../resource-policy)も適用します。JavaScript 入力は Rust の呼び出し前に検証します。

## 関連メソッド

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
