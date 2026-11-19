---
locale: ko
source_revision: e6b8dfbe3d3140a4697ec5a94f4d508f975bc9c0847e91c70ef29e95978ddcfb
translation_provenance: en-US-korean-reviewed
api_id: "ErrorKind"
---
# ErrorKind

안정적인 라이브러리 오류 종류를 설명합니다.

```rust
pub enum ErrorKind
```

## 매개변수, 반환값, 일반적인 사용법

열거형에는 11개 변형이 있으며 Rust에서 복사와 순서 비교가 가능합니다. 브라우저 ErrorKind는 Rust에서 읽은 고정된 이름 맵입니다. 특정 오류는 메시지 비교보다 종류 포함 여부로 검사하세요. 브라우저의 명명된 생성자는 message, matches(), kinds(), with_kind()를 제공하는 해제 가능한 Rust 값을 만듭니다. 라이브러리가 던지는 예외는 category="library"와 kinds 배열을 가진 네이티브 JS Error이며 e.kinds.includes(ErrorKind.Unmarshal)로 검사합니다. parameter/resource/moved JavaScript 오류는 별개이며 라이브러리 종류가 없습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind" locale="ko" />

## 관련 메서드

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
