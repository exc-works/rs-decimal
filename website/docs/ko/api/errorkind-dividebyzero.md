---
locale: ko
source_revision: c5bc8b97bab18c557c0918fb7ad15cabc2fd824c618c469b8f007d1f6299272b
translation_provenance: en-US-korean-reviewed
api_id: "ErrorKind::DivideByZero"
---
# ErrorKind::DivideByZero

DivideByZero 오류를 식별합니다.

```rust
DivideByZero
```

## 매개변수, 반환값, 일반적인 사용법

제수가 0입니다. ErrorKind는 메시지 문자열이 아니라 안정적인 분류입니다. 브라우저 열거값은 Rust 변형 이름입니다. 생성한 오류는 matches()를 사용하고 잡은 bridge 예외는 kinds 배열을 사용합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-dividebyzero" locale="ko" />

## 관련 메서드

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
