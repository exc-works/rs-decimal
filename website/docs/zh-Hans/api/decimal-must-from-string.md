---
locale: zh-Hans
source_revision: bb2070c90fbcf81b9f15485debb923fef5e838831e150577f2d283f059db73b6
api_id: Decimal::must_from_string
---
# Decimal::must_from_string

在非法输入属于程序错误的场景下解析可信的小数文本。

```rust
fn must_from_string (text : & str) -> Self
```

## 参数、返回值及注意事项

text: &str/string，返回 Decimal。合法输入与 new_from_string 得到相同状态。非法语法在 Rust 中触发包含 DecimalError 的 panic；编译后的 Wasm 发生 trap，不会返回可恢复的库错误。用户输入应优先使用 new_from_string。未捕获的 trap 会结束当前运行；再次使用 Rust 前应开始全新的运行。参数及资源检查仍会在调用 Rust 前拒绝超限输入。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-decimal-must-from-string" locale="zh-Hans" />

## 边界

bridge 保留 Rust 方法名及参数顺序。自由函数在 Playground 中可直接调用。借用的输入不会被消耗或改变；bridge 拒绝失效或其他运行实例的对象。全部操作遵守 [资源限制](../resource-policy)。

## 相关方法

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
