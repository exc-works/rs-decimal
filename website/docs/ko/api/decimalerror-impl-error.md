---
locale: ko
source_revision: d94c5a1a48ed0fbe522456d88bc4774ce805cace3b32e2a5428e2527dc3be2ce
translation_provenance: en-US-korean-reviewed
api_id: "DecimalError::impl Error"
---
# DecimalError::impl Error

Rust Error 구현을 사용합니다.

```rust
impl Error for DecimalError
```

## 매개변수, 반환값, 일반적인 사용법

박스에 담은 오류 전파를 포함해 표준 Rust Error로 사용합니다. 추가 종류는 연결된 원인이 아니며 source()는 None입니다. 네이티브 Rust 트레이트 문법이며 브라우저는 메서드로 공개하지 않습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-impl-error" locale="ko" />

## 브라우저 제한

SQL 텍스트/바이트 입력과 오류 메시지는 UTF-8 8192바이트까지입니다. SQL 텍스트는 바깥 따옴표 제거 후 [리소스 검사](../resource-policy)를 받으므로 따옴표로 지수 제한을 우회할 수 없습니다. 빌림 호출은 입력을 유지하며 소비되거나 해제된 핸들은 재사용할 수 없습니다. Rust 트레이트/가져오기 예제는 네이티브 테스트에서 실행하며 브라우저 메서드 지원을 뜻하지 않습니다.

## 관련 메서드

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
