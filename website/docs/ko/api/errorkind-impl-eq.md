---
locale: ko
source_revision: 001b8b003c099ab66879c97c5d63d2105fa698bcbc3c15c760670300d17a20a3
translation_provenance: en-US-korean-reviewed
api_id: "ErrorKind::impl Eq"
---
# ErrorKind::impl Eq

Rust Eq 구현을 사용합니다.

```rust
impl Eq for ErrorKind
```

## 매개변수, 반환값, 일반적인 사용법

동등성은 전체 오류 메시지/종류 또는 정확한 열거 변형을 비교합니다. DecimalError 동등성은 하나의 종류에 일치하는 것보다 엄격합니다. 네이티브 Rust 트레이트 문법이며 브라우저는 메서드로 공개하지 않습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-impl-eq" locale="ko" />

## 관련 메서드

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
