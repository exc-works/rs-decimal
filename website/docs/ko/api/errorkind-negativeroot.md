---
locale: ko
source_revision: e2bc3815435c7e993973dc27c3f32515fd48346d012f56633ed09feff4f0aa99
translation_provenance: en-US-korean-reviewed
api_id: "ErrorKind::NegativeRoot"
---
# ErrorKind::NegativeRoot

NegativeRoot 오류를 식별합니다.

```rust
NegativeRoot
```

## 매개변수, 반환값, 일반적인 사용법

음수 값의 짝수 차수 근을 요청했습니다. ErrorKind는 메시지 문자열이 아니라 안정적인 분류입니다. 브라우저 열거값은 Rust 변형 이름입니다. 생성한 오류는 matches()를 사용하고 잡은 bridge 예외는 kinds 배열을 사용합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-negativeroot" locale="ko" />

## 관련 메서드

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
