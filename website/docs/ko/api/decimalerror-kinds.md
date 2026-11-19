---
locale: ko
source_revision: a985525078f122e92ef2d70bf9a962a63b82b868ba67925f976407356bf4ff6b
translation_provenance: en-US-korean-reviewed
api_id: "DecimalError::kinds"
---
# DecimalError::kinds

모든 오류 종류를 읽습니다.

```rust
fn kinds (& self) -> & [ErrorKind]
```

## 매개변수, 반환값, 일반적인 사용법

매개변수 없음. &[ErrorKind]/독립된 변형 이름 배열을 반환합니다. 종류는 중복을 제거하고 알파벳순이 아니라 Rust 열거형 순서로 정렬합니다. 브라우저의 명명된 생성자는 message, matches(), kinds(), with_kind()를 제공하는 해제 가능한 Rust 값을 만듭니다. 라이브러리가 던지는 예외는 category="library"와 kinds 배열을 가진 네이티브 JS Error이며 e.kinds.includes(ErrorKind.Unmarshal)로 검사합니다. parameter/resource/moved JavaScript 오류는 별개이며 라이브러리 종류가 없습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-kinds" locale="ko" />

## 관련 메서드

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
