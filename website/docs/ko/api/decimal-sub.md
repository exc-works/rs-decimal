---
locale: ko
source_revision: db43c880650b6eb59e9c805ec0cdb77a4879108bb98a5d2e10ec38f65ac55f46
translation_provenance: en-US-korean-reviewed
api_id: Decimal::sub
---
# Decimal::sub

소수를 정확하게 뺍니다.

```rust
fn sub (& self , other : & Self) -> Self
```

## 매개변수, 반환값, 일반적인 사용법

other: &Decimal. 두 피연산자 중 큰 소수 자릿수로 새 Decimal을 반환합니다. 자릿수 맞춤은 정확하며 숫자를 버리지 않습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-sub" locale="ko" />

## 관련 메서드

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
