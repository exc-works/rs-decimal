---
locale: ko
source_revision: 51d8eb06be2cf1b2c76ae713dc34443717bd6adc00c52dec95e805914eed6ec1
translation_provenance: en-US-korean-reviewed
api_id: "DecimalError::matches"
---
# DecimalError::matches

오류 종류 포함 여부를 검사합니다.

```rust
fn matches (& self , kind : ErrorKind) -> bool
```

## 매개변수, 반환값, 일반적인 사용법

kind: ErrorKind. bool/boolean을 반환합니다. 빌리기는 오류를 바꾸지 않습니다. 한 오류가 여러 종류에 동시에 해당할 수 있습니다. 브라우저의 명명된 생성자는 message, matches(), kinds(), with_kind()를 제공하는 해제 가능한 Rust 값을 만듭니다. 라이브러리가 던지는 예외는 category="library"와 kinds 배열을 가진 네이티브 JS Error이며 e.kinds.includes(ErrorKind.Unmarshal)로 검사합니다. parameter/resource/moved bridge 오류는 별개이며 라이브러리 종류가 없습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-matches" locale="ko" />

## 관련 메서드

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
