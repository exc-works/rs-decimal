---
locale: zh-Hans
source_revision: 5b92e8602e178e484bc5b465cd3d2014888bc1d25a5a0633a6502326ad20ca84
api_id: "ErrorKind::RoundUnnecessary"
---
# ErrorKind::RoundUnnecessary

识别 RoundUnnecessary 错误。

```rust
RoundUnnecessary
```

## 参数、返回值及常规用法

要求精确舍入，但会丢弃非零数字。ErrorKind 是稳定类别，不是错误消息文本。浏览器枚举值为 Rust 变体名；构造的错误使用 matches()，捕获的 bridge 异常提供 kinds 数组。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-roundunnecessary" locale="zh-Hans" />

## 浏览器边界

SQL 文本及字节输入、错误消息最多 8192 个 UTF-8 字节。SQL 去除外围引号后的文本仍经过 [十进制资源检查](../resource-policy)，不能借引号绕过指数限制。借用操作不消耗输入；消耗后或释放后的句柄不能复用。原生 Rust trait 和导入示例由原生测试执行，不计作浏览器方法。

## 相关方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
