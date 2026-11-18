---
locale: ko
source_revision: 37fcbc194399d8059373fc9c0c61833d997ffb41871435eb1c6867b7870d1acc
translation_provenance: en-US-korean-reviewed
api_id: Decimal::impl Clone
---
# Decimal::impl Clone

전체 표현을 유지하며 소수를 복제합니다.

```rust
impl Clone for Decimal
```

## 일반적인 사용법과 의미

Clone은 계수, 소수 자릿수, 초기화 상태를 복사합니다. JavaScript에서는 Rust 트레이트 대신 이름이 있는 clone() 어댑터를 사용합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-clone" locale="ko" />

이 Rust 문법 예제는 네이티브 테스트에서 실행하며 브라우저의 트레이트 지원을 뜻하지 않습니다.

## 관련 문서

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
