---
locale: zh-Hans
source_revision: cc52467a55bcaa7e86e52d9cf523d5b8a22924e126619052d59e51cd0b6aab8b
api_id: Decimal::into_coefficient
---
# Decimal::into_coefficient

移动出未缩放的整数系数。

```rust
fn into_coefficient (self) -> BigInt
```

## 参数、返回值及常规用法

无参数；Rust 返回 BigInt，JS 返回 bigint。丢弃小数位数和初始化状态，因此 1.2300 得到 12300。默认零得到 0。Decimal 被消耗，不能再次读取；JavaScript 中需要不消耗的快照时使用 coefficient。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-into-coefficient" locale="zh-Hans" />

## 相关方法

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
