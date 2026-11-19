---
locale: ja
source_revision: 64d94a480e185cbb38c975ef60712d5ae4ac373961962b675392db6ef9375f09
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::remainder
---
# Decimal::remainder

整数部分とスケール適用前の小数部分の剰余に分けます。

```rust
fn remainder (& self) -> (BigInt , BigInt)
```

## 引数、戻り値、通常の使い方

引数なし。(BigInt, BigInt) を返し、[bigint, bigint] に対応します。第 1 要素はゼロ方向に切り捨てた値、第 2 要素は符号付きの係数剰余であり、Decimal の小数部分ではありません。-12.750 の結果は (-12, -750) です。第 2 要素を -0.750 と解釈するには元のスケール 3 が必要です。既定のゼロは (0, 0) を返します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-remainder" locale="ja" />

## 関連メソッド

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
