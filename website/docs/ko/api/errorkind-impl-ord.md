---
locale: ko
source_revision: 77c1d26d73dd77cab1d0c87534f9e5ec0b5d57b39f76087cd3ef2b288f8c6fae
translation_provenance: en-US-korean-reviewed
api_id: "ErrorKind::impl Ord"
---
# ErrorKind::impl Ord

Rust Ord 구현을 사용합니다.

```rust
impl Ord for ErrorKind
```

## 매개변수, 반환값, 일반적인 사용법

ErrorKind를 심각도나 이름의 알파벳 순서가 아니라 Rust 선언 순서로 정렬합니다. 네이티브 Rust 트레이트 문법이며 브라우저는 메서드로 공개하지 않습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-impl-ord" locale="ko" />

## 관련 메서드

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
