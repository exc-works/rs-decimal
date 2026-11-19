---
locale: zh-Hans
source_revision: da8cebe24a220cbee0329a50db3c796d6390c270eb7d574cffaac17c4ddc1264
api_id: "NullDecimal::impl std :: fmt :: Display::fmt"
---
# NullDecimal::impl std :: fmt :: Display::fmt

通过 Rust Display 格式化 nullable 值。

```rust
fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result
```

## 参数、返回值及常规用法

Display 委托 nullable string()：有效时输出内部规范文本，无效时输出 null。格式器写入文本，不是 JSON token 或字节缓冲区。format! 调用 Display::fmt；此 trait 语法在原生环境执行，不是浏览器方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-nulldecimal-impl-std-fmt-display-fmt" locale="zh-Hans" />

## 相关方法

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
