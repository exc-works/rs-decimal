---
locale: zh-Hant
source_revision: 3430467a0f3b43857ec837a38512411503e4016266c068e3065f8bd60687547d
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::try_from_coefficient
---
# Decimal::try_from_coefficient

透過擁有所有權的係數和小數位數建構已初始化值。

```rust
fn try_from_coefficient (coefficient : BigInt , scale : usize) -> Result < Self , DecimalError >
```

## 參數、回傳值及常規用法

coefficient: BigInt；scale: usize（非負小數位數）。保留末尾零和零值的精度，不使用文字解析器的正規化。Rust 在超過 isize::MAX 時回傳 Overflow。瀏覽器使用精確 bigint 和 wasm32 usize 範圍內的整數 number；超過 i32::MAX 的 scale 保留 Overflow，其他超過 1000 的 scale 觸及 playground 資源限制。JS bigint 不可變，因此不是被移動的包裝物件。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-try-from-coefficient" locale="zh-Hant" />

## 相關方法

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
