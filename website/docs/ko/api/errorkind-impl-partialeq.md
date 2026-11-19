---
locale: ko
source_revision: 5ee0e7b51aa068361991bd8a8b6016ae9f25f271ec57ec1d0990b39445f68b64
translation_provenance: en-US-korean-reviewed
api_id: "ErrorKind::impl PartialEq"
---
# ErrorKind::impl PartialEq

Rust PartialEq 구현을 사용합니다.

```rust
impl PartialEq for ErrorKind
```

## 매개변수, 반환값, 일반적인 사용법

동등성은 전체 오류 메시지/종류 또는 정확한 열거 변형을 비교합니다. DecimalError 동등성은 하나의 종류에 일치하는 것보다 엄격합니다. 네이티브 Rust 트레이트 문법이며 브라우저는 메서드로 공개하지 않습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-impl-partialeq" locale="ko" />

## 관련 메서드

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
