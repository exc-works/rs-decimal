---
locale: zh-Hans
source_revision: 79298c474720d5ca067e6e36ecdefe721611687b58bab5fff1d35e1889208bcd
api_id: "NullDecimal::value"
---
# NullDecimal::value

获取规范的 SQL 值。

```rust
fn value (& self) -> Option < String >
```

## 参数、返回值及常规用法

无参数，返回 `Option<String>`/string | null。无效 nullable 返回 None/null，与隐藏内部值无关。有效值返回规范小数文本，内部零未初始化时也返回 "0"。移除小数末尾零，但不改变 scale 或初始化状态。返回文本是标量值，不是带引号的 SQL 查询片段。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-nulldecimal-value" locale="zh-Hans" />

## 浏览器边界

SQL 文本及字节输入、错误消息最多 8192 个 UTF-8 字节。SQL 去除外围引号后的文本仍经过 [十进制资源检查](../resource-policy)，不能借引号绕过指数限制。借用操作不消耗输入；消耗后或释放后的句柄不能复用。原生 Rust trait 和导入示例由原生测试执行，不计作浏览器方法。

## 相关方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
