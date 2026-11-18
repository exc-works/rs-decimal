---
locale: ko
source_revision: 150ae4511881de6cce3960793291dd735b00fc7aa5175f86d5957a93c70d0c13
translation_provenance: en-US-korean-reviewed
api_id: Decimal::impl Debug
---
# Decimal::impl Debug

Rust 디버깅에서 소수를 확인합니다.

```rust
impl Debug for Decimal
```

## 일반적인 사용법과 의미

파생된 진단 표현은 내부 필드를 포함하며 안정적인 직렬화 형식이 아닙니다. 브라우저 출력은 명시적인 스냅샷을 사용합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-debug" locale="ko" />

이 Rust 문법 예제는 네이티브 테스트에서 실행하며 브라우저의 트레이트 지원을 뜻하지 않습니다.

## 관련 문서

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
