---
locale: ja
source_revision: 0ad97f33b356cd281b88951f90d8d3c10dff06f728f22f7136b85c34bc769e4e
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::precision
---
# Decimal::precision

値に保存された小数点以下の桁数を読み取ります。

```rust
fn precision (& self) -> isize
```

## 引数、戻り値、使用上の注意

isize（ブラウザーでは整数の number）を返します。有効数字の数ではなくスケールであり、末尾のゼロも数えます。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-precision" locale="ja" />

## 関連メソッド

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
