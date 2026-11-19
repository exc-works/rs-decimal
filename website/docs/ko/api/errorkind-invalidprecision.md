---
locale: ko
source_revision: 68b84662c55e7239d15b1fe54a5a2076eb45cea41916719d90f7dbe8ab9e7150
translation_provenance: en-US-korean-reviewed
api_id: "ErrorKind::InvalidPrecision"
---
# ErrorKind::InvalidPrecision

InvalidPrecision 오류를 식별합니다.

```rust
InvalidPrecision
```

## 매개변수, 반환값, 일반적인 사용법

요청한 정밀도가 잘못되었습니다. ErrorKind는 메시지 문자열이 아니라 안정적인 분류입니다. 브라우저 열거값은 Rust 변형 이름입니다. 생성한 오류는 matches()를 사용하고 잡은 bridge 예외는 kinds 배열을 사용합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-invalidprecision" locale="ko" />

## 관련 메서드

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
