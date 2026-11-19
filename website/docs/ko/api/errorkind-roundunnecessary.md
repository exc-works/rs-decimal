---
locale: ko
source_revision: 189b6d1ee3eb48467bf982330ec482f99ea7cd30e80f4f3c225f2808c0e03cc3
translation_provenance: en-US-korean-reviewed
api_id: "ErrorKind::RoundUnnecessary"
---
# ErrorKind::RoundUnnecessary

RoundUnnecessary 오류를 식별합니다.

```rust
RoundUnnecessary
```

## 매개변수, 반환값, 일반적인 사용법

정확한 반올림을 요청했지만 0이 아닌 숫자를 버려야 합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-roundunnecessary" locale="ko" />

## 관련 메서드

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
