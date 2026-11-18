---
locale: ko
source_revision: c136bea39fff64039ef208c269a0655f2c126075df944740bc4d24f9d953ef55
translation_provenance: en-US-korean-reviewed
api_id: "ErrorKind"
---
# ErrorKind

안정적인 라이브러리 오류 종류를 설명합니다.

```rust
pub enum ErrorKind
```

## 매개변수, 반환값, 일반적인 사용법

열거형에는 11개 변형이 있으며 Rust에서 복사와 순서 비교가 가능합니다. 브라우저 ErrorKind는 Rust에서 읽은 고정된 이름 맵입니다. 특정 오류는 메시지 비교보다 종류 포함 여부로 검사하세요. 브라우저의 명명된 생성자는 message, matches(), kinds(), with_kind()를 제공하는 해제 가능한 Rust 값을 만듭니다. 라이브러리가 던지는 예외는 category="library"와 kinds 배열을 가진 네이티브 JS Error이며 e.kinds.includes(ErrorKind.Unmarshal)로 검사합니다. parameter/resource/moved bridge 오류는 별개이며 라이브러리 종류가 없습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind" locale="ko" />

## 브라우저 제한

SQL 텍스트/바이트 입력과 오류 메시지는 UTF-8 8192바이트까지입니다. SQL 텍스트는 바깥 따옴표 제거 후 [리소스 검사](../resource-policy)를 받으므로 따옴표로 지수 제한을 우회할 수 없습니다. 빌림 호출은 입력을 유지하며 소비되거나 해제된 핸들은 재사용할 수 없습니다. Rust 트레이트/가져오기 예제는 네이티브 테스트에서 실행하며 브라우저 메서드 지원을 뜻하지 않습니다.

## 관련 메서드

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
