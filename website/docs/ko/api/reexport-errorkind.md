---
locale: ko
source_revision: 76aa255f8bc8b60ca22e7c6936a048607e56eb1eaa641962626233731b182883
translation_provenance: en-US-korean-reviewed
api_id: "reexport::ErrorKind"
---
# reexport::ErrorKind

크레이트 루트에서 ErrorKind을 가져옵니다.

```rust
pub use error::ErrorKind as ErrorKind
```

## 매개변수, 반환값, 일반적인 사용법

같은 공개 항목에 rs_decimal::ErrorKind을 사용합니다. 가져오기 경로이며 별도 구현이나 브라우저 메서드가 아닙니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-reexport-errorkind" locale="ko" />

## 브라우저 제한

SQL 텍스트/바이트 입력과 오류 메시지는 UTF-8 8192바이트까지입니다. SQL 텍스트는 바깥 따옴표 제거 후 [리소스 검사](../resource-policy)를 받으므로 따옴표로 지수 제한을 우회할 수 없습니다. 빌림 호출은 입력을 유지하며 소비되거나 해제된 핸들은 재사용할 수 없습니다. Rust 트레이트/가져오기 예제는 네이티브 테스트에서 실행하며 브라우저 메서드 지원을 뜻하지 않습니다.

## 관련 메서드

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
