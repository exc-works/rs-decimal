---
locale: ko
source_revision: c456812ef50493713a2bbd81d6c57db5a0f730099b5878e83b3d4f358245d822
translation_provenance: en-US-korean-reviewed
api_id: "ErrorKind::InvalidLog"
---
# ErrorKind::InvalidLog

InvalidLog 오류를 식별합니다.

```rust
InvalidLog
```

## 매개변수, 반환값, 일반적인 사용법

로그 입력이 양수가 아닙니다. ErrorKind는 메시지 문자열이 아니라 안정적인 분류입니다. 브라우저 열거값은 Rust 변형 이름입니다. 생성한 오류는 matches()를 사용하고 잡은 bridge 예외는 kinds 배열을 사용합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-invalidlog" locale="ko" />

## 관련 메서드

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
