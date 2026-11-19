---
locale: ko
source_revision: b6a1d6732343e8fec8b5e4584b880016ecb9ffdc8cf16c927c04637de618fc19
translation_provenance: en-US-korean-reviewed
api_id: "ErrorKind::impl PartialOrd"
---
# ErrorKind::impl PartialOrd

Rust PartialOrd 구현을 사용합니다.

```rust
impl PartialOrd for ErrorKind
```

## 매개변수, 반환값, 일반적인 사용법

ErrorKind를 심각도나 이름의 알파벳 순서가 아니라 Rust 선언 순서로 정렬합니다. 네이티브 Rust 트레이트 문법이며 브라우저는 메서드로 공개하지 않습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-impl-partialord" locale="ko" />

## 관련 메서드

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
