---
locale: ko
source_revision: 7eeabc8ee1ace4f862b185d7c83f7d0a53594b9ff67ac15d988dde4ffaa12e4b
translation_provenance: en-US-korean-reviewed
api_id: "ScanValue::impl Copy"
---
# ScanValue::impl Copy

Rust Copy 구현을 사용합니다.

```rust
impl Copy for ScanValue
```

## 매개변수, 반환값, 일반적인 사용법

Copy는 작은 열거형을 복제하며 빌린 스칼라 데이터를 깊이 복사하지 않습니다. 네이티브 Rust 트레이트 문법이며 브라우저는 메서드로 공개하지 않습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-impl-copy" locale="ko" />

## 관련 메서드

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
