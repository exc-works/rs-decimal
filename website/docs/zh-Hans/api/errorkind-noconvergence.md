---
locale: zh-Hans
source_revision: 7a9119eb74e99c5e35f09e87df2556b438ba2f0ad510d5e668cdcc13dc40e550
api_id: "ErrorKind::NoConvergence"
---
# ErrorKind::NoConvergence

识别 NoConvergence 错误。

```rust
NoConvergence
```

## 参数、返回值及常规用法

有界算法无法确认舍入结果；不表示数学结果不存在。ErrorKind 是稳定类别，不是错误消息文本。浏览器枚举值为 Rust 变体名；构造的错误使用 matches()，捕获的 bridge 异常提供 kinds 数组。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-noconvergence" locale="zh-Hans" />

## 浏览器边界

SQL 文本及字节输入、错误消息最多 8192 个 UTF-8 字节。SQL 去除外围引号后的文本仍经过 [十进制资源检查](../resource-policy)，不能借引号绕过指数限制。借用操作不消耗输入；消耗后或释放后的句柄不能复用。原生 Rust trait 和导入示例由原生测试执行，不计作浏览器方法。

## 相关方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
