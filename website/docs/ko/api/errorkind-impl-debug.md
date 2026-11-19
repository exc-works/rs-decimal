---
locale: ko
source_revision: 1b410a44c40ddf1ed7ab08ecafd94bf955a8c6e8ace55716beba7530f0db9122
translation_provenance: en-US-korean-reviewed
api_id: "ErrorKind::impl Debug"
---
# ErrorKind::impl Debug

Rust Debug 구현을 사용합니다.

```rust
impl Debug for ErrorKind
```

## 매개변수, 반환값, 일반적인 사용법

Debug는 진단 텍스트이며 안정적인 전송 직렬화가 아닙니다. 네이티브 Rust 트레이트 문법이며 브라우저는 메서드로 공개하지 않습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-impl-debug" locale="ko" />

## 관련 메서드

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
