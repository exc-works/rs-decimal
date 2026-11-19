---
locale: ko
source_revision: 11ee599a7357841e984cdaecadf4da635302996e14ae33191c323e57e59262ff
translation_provenance: en-US-korean-reviewed
api_id: "ScanValue::impl Clone"
---
# ScanValue::impl Clone

Rust Clone 구현을 사용합니다.

```rust
impl Clone for ScanValue
```

## 매개변수, 반환값, 일반적인 사용법

Clone은 전체 값을 유지하지만 빌린 ScanValue 슬라이스는 여전히 원래 데이터를 참조합니다. 네이티브 Rust 트레이트 문법이며 브라우저는 메서드로 공개하지 않습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-impl-clone" locale="ko" />

## 관련 메서드

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
