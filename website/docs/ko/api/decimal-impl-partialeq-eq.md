---
locale: ko
source_revision: 1d252fa81f50f2414d333721b6e7df59a5331a4689705908696e9896f6fb4dbf
translation_provenance: en-US-korean-reviewed
api_id: Decimal::impl PartialEq::eq
---
# Decimal::impl PartialEq::eq

Rust 연산자 또는 PartialEq로 수치의 동등성을 비교합니다.

```rust
fn eq (& self , other : & Self) -> bool
```

## 일반적인 사용법과 의미

eq는 두 피연산자를 빌려 bool을 반환합니다. 12.00 == 12.0입니다. JavaScript는 equal()을 사용하며 ===는 래퍼 동일성을 검사합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-partialeq-eq" locale="ko" />

이 Rust 문법 예제는 네이티브 테스트에서 실행하며 브라우저의 트레이트 지원을 뜻하지 않습니다.

## 관련 문서

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
