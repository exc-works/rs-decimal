---
locale: ko
source_revision: 52cfb6e6f9d640e445d0e6b774455d4a9cdc44bf1d1bc94d3efd717749548fc4
translation_provenance: en-US-korean-reviewed
api_id: "DecimalError::new"
---
# DecimalError::new

하나의 종류를 가진 오류를 만듭니다.

```rust
fn new (kind : ErrorKind , message : impl Into < String >) -> Self
```

## 매개변수, 반환값, 일반적인 사용법

kind: ErrorKind, message: impl `Into<String>`/string. 새 DecimalError를 반환합니다. 메시지는 사람이 읽기 위한 것으로 종류 포함 여부를 결정하지 않습니다. 브라우저의 명명된 생성자는 message, matches(), kinds(), with_kind()를 제공하는 해제 가능한 Rust 값을 만듭니다. 라이브러리가 던지는 예외는 category="library"와 kinds 배열을 가진 네이티브 JS Error이며 e.kinds.includes(ErrorKind.Unmarshal)로 검사합니다. parameter/resource/moved JavaScript 오류는 별개이며 라이브러리 종류가 없습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-new" locale="ko" />

## 관련 메서드

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
