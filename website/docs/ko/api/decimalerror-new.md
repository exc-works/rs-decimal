---
locale: ko
source_revision: c70b9c986ae1c2d1db6f94784745de5a4d3bb8a7cf3f6af3550e9be48e0f551f
translation_provenance: en-US-korean-reviewed
api_id: "DecimalError::new"
---
# DecimalError::new

하나의 종류를 가진 오류를 만듭니다.

```rust
fn new (kind : ErrorKind , message : impl Into < String >) -> Self
```

## 매개변수, 반환값, 일반적인 사용법

kind: ErrorKind, message: impl `Into<String>`/string. 새 DecimalError를 반환합니다. 메시지는 사람이 읽기 위한 것으로 종류 포함 여부를 결정하지 않습니다. 브라우저의 명명된 생성자는 message, matches(), kinds(), with_kind()를 제공하는 해제 가능한 Rust 값을 만듭니다. 라이브러리가 던지는 예외는 category="library"와 kinds 배열을 가진 네이티브 JS Error이며 e.kinds.includes(ErrorKind.Unmarshal)로 검사합니다. parameter/resource/moved bridge 오류는 별개이며 라이브러리 종류가 없습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-new" locale="ko" />

## 브라우저 제한

SQL 텍스트/바이트 입력과 오류 메시지는 UTF-8 8192바이트까지입니다. SQL 텍스트는 바깥 따옴표 제거 후 [리소스 검사](../resource-policy)를 받으므로 따옴표로 지수 제한을 우회할 수 없습니다. 빌림 호출은 입력을 유지하며 소비되거나 해제된 핸들은 재사용할 수 없습니다. Rust 트레이트/가져오기 예제는 네이티브 테스트에서 실행하며 브라우저 메서드 지원을 뜻하지 않습니다.

## 관련 메서드

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
