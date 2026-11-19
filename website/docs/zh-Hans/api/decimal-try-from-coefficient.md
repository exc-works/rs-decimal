---
locale: zh-Hans
source_revision: 3430467a0f3b43857ec837a38512411503e4016266c068e3065f8bd60687547d
api_id: Decimal::try_from_coefficient
---
# Decimal::try_from_coefficient

通过拥有所有权的系数和小数位数构造已初始化值。

```rust
fn try_from_coefficient (coefficient : BigInt , scale : usize) -> Result < Self , DecimalError >
```

## 参数、返回值及常规用法

coefficient: BigInt；scale: usize（非负小数位数）。保留末尾零和零值的精度，不使用文本解析器的规范化。Rust 在超过 isize::MAX 时返回 Overflow。浏览器使用精确 bigint 和 wasm32 usize 范围内的整数 number；超过 i32::MAX 的 scale 保留 Overflow，其他超过 1000 的 scale 触及 playground 资源限制。JS bigint 不可变，因此不是被移动的包装对象。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-try-from-coefficient" locale="zh-Hans" />

## 相关方法

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
