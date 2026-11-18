---
locale: zh-Hans
source_revision: 174ec0afec71ca2dec4f0c9c844589a1e01782200d059f787f800837923cca1b
api_id: "ScanValue::Unsupported"
---
# ScanValue::Unsupported

选择 Unsupported SQL 输入变体。

```rust
Unsupported
```

## 参数、返回值及常规用法

不做强制转换，明确报告不支持的驱动输入。scan 返回 Unmarshal 并保留原值。scan 借用变体，同一次浏览器运行中可复用；拒绝伪造或其他运行实例的值。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-unsupported" locale="zh-Hans" />

## 浏览器边界

SQL 文本及字节输入、错误消息最多 8192 个 UTF-8 字节。SQL 去除外围引号后的文本仍经过 [十进制资源检查](../resource-policy)，不能借引号绕过指数限制。借用操作不消耗输入；消耗后或释放后的句柄不能复用。原生 Rust trait 和导入示例由原生测试执行，不计作浏览器方法。

## 相关方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
