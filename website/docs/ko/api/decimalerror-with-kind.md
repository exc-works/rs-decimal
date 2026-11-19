---
locale: ko
source_revision: ecc694dc9095e4bf91fee90f15b14f69b82899c7811b1a6abfdda76dd5e06c56
translation_provenance: en-US-korean-reviewed
api_id: "DecimalError::with_kind"
---
# DecimalError::with_kind

오류를 소비하고 종류를 추가합니다.

```rust
fn with_kind (mut self , kind : ErrorKind) -> Self
```

## 매개변수, 반환값, 일반적인 사용법

kind: ErrorKind. self를 소비하고 새 DecimalError를 반환합니다. 기존 종류와 메시지를 유지하고 중복은 무시합니다. 원래 브라우저 핸들은 더 이상 사용할 수 없습니다. 잘못된 JS kind는 이동 전에 거부합니다. 브라우저의 명명된 생성자는 message, matches(), kinds(), with_kind()를 제공하는 해제 가능한 Rust 값을 만듭니다. 라이브러리가 던지는 예외는 category="library"와 kinds 배열을 가진 네이티브 JS Error이며 e.kinds.includes(ErrorKind.Unmarshal)로 검사합니다. parameter/resource/moved bridge 오류는 별개이며 라이브러리 종류가 없습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-with-kind" locale="ko" />

## 관련 메서드

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
