---
locale: zh-Hans
source_revision: 7805847022fc73cbb47e3960021e981cfb8ac4ab47a7b0a542ffa809d986fb9d
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

## 相关方法

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
