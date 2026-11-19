---
locale: ko
source_revision: 368adfaeea00a469d274e0e87fc3e84f66ca2848d32cfd8759858b2bac06fe99
translation_provenance: en-US-korean-reviewed
api_id: "ErrorKind::Overflow"
---
# ErrorKind::Overflow

Overflow 오류를 식별합니다.

```rust
Overflow
```

## 매개변수, 반환값, 일반적인 사용법

계수, 소수 자릿수 또는 연산이 코어 지원 범위를 초과합니다. ErrorKind는 메시지 문자열이 아니라 안정적인 분류입니다. 브라우저 열거값은 Rust 변형 이름입니다. 생성한 오류는 matches()를 사용하고 잡은 bridge 예외는 kinds 배열을 사용합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-overflow" locale="ko" />

## 관련 메서드

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
