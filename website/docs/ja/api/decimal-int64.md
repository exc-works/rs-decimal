---
locale: ja
source_revision: f83d4eab7c44d20bd9f7f112876d8de08078fd38cb0bbaaf09e60fe2891d5d1e
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::int64
---
# Decimal::int64

成功フラグとともに固定幅整数へ変換します。

```rust
fn int64 (& self) -> (i64 , bool)
```

## 引数、戻り値、通常の使い方

引数なし。(i64, bool) を返し、[bigint, boolean] に対応します。範囲内の整数値は整数と true を返します。小数やオーバーフローは切り捨て値や例外ではなく (0, false) を返します。スケールだけでは整数か判定できず、12.00 は成功します。失敗と有効なゼロを区別するため、必ずフラグを確認してください。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-int64" locale="ja" />

## 関連メソッド

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
