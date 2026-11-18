---
locale: ko
source_revision: 4b0752061c959ac5ce6e3b7af4bf7c9232d0d1cfb43869372b4caa7a5d6f49f4
translation_provenance: en-US-korean-reviewed
api_id: NullDecimal::impl Debug
---
# NullDecimal::impl Debug

Rust Debug로 nullable 상태를 확인합니다.

```rust
impl Debug for NullDecimal
```

## 일반적인 사용법과 의미

진단 텍스트에는 유효성과 저장 필드가 들어가며 숨겨진 값도 포함합니다. 안정적인 전송 형식이 아니므로 브라우저에서는 스냅샷을 사용합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-nulldecimal-impl-debug" locale="ko" />

이 Rust 문법 예제는 네이티브 테스트에서 실행하며 브라우저의 트레이트 지원을 뜻하지 않습니다.

## 관련 문서

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
