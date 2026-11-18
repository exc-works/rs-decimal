---
locale: ko
source_revision: 3ced5d99beade676205267c80e93f90c49f974486393990035a12b18f4061c4a
translation_provenance: en-US-korean-reviewed
api_id: Decimal::impl PartialEq
---
# Decimal::impl PartialEq

Rust 연산자 또는 PartialEq로 수치의 동등성을 비교합니다.

```rust
impl PartialEq for Decimal
```

## 일반적인 사용법과 의미

eq는 두 피연산자를 빌려 bool을 반환합니다. 12.00 == 12.0입니다. JavaScript는 equal()을 사용하며 ===는 래퍼 동일성을 검사합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-partialeq" locale="ko" />

이 Rust 문법 예제는 네이티브 테스트에서 실행하며 브라우저의 트레이트 지원을 뜻하지 않습니다.

## 관련 문서

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
