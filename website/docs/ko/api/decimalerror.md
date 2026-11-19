---
locale: ko
source_revision: 51df12e136a0b4ed98ea9f557c4e61c7872827fb92cbe6a97475d66177b43284
translation_provenance: en-US-korean-reviewed
api_id: "DecimalError"
---
# DecimalError

메시지와 하나 이상의 라이브러리 오류 종류를 유지합니다.

```rust
pub struct DecimalError
```

## 매개변수, 반환값, 일반적인 사용법

DecimalError는 읽기 쉬운 메시지와 순서가 있고 중복이 제거된 종류를 저장합니다. with_kind는 값을 소비하고 matches와 kinds는 빌립니다. Rust Error와 Display를 구현합니다. 브라우저의 명명된 생성자는 message, matches(), kinds(), with_kind()를 제공하는 해제 가능한 Rust 값을 만듭니다. 라이브러리가 던지는 예외는 category="library"와 kinds 배열을 가진 네이티브 JS Error이며 e.kinds.includes(ErrorKind.Unmarshal)로 검사합니다. parameter/resource/moved JavaScript 오류는 별개이며 라이브러리 종류가 없습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror" locale="ko" />

## 관련 메서드

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
