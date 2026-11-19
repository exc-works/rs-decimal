---
locale: ko
source_revision: dc7af33554badb1c95cd11064df3b51faeaca3112afc1b82cf2efcaf4ff1844d
translation_provenance: en-US-korean-reviewed
api_id: "ErrorKind::RoundUnnecessary"
---
# ErrorKind::RoundUnnecessary

RoundUnnecessary 오류를 식별합니다.

```rust
RoundUnnecessary
```

## 매개변수, 반환값, 일반적인 사용법

정확한 반올림을 요청했지만 0이 아닌 숫자를 버려야 합니다. ErrorKind는 메시지 문자열이 아니라 안정적인 분류입니다. 브라우저 열거값은 Rust 변형 이름입니다. 생성한 오류는 matches()를 사용하고 잡은 bridge 예외는 kinds 배열을 사용합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-roundunnecessary" locale="ko" />

## 관련 메서드

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
