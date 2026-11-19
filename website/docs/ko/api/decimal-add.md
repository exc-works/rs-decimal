---
locale: ko
source_revision: 1edc6ba7e191babd98a0717864d8696548b2086ee97162b567ea1d847a357dad
translation_provenance: en-US-korean-reviewed
api_id: Decimal::add
---
# Decimal::add

소수를 정확하게 더합니다.

```rust
fn add (& self , other : & Self) -> Self
```

## 매개변수, 반환값, 일반적인 사용법

other: &Decimal. 두 피연산자 중 큰 소수 자릿수로 새 Decimal을 반환합니다. 자릿수 맞춤은 정확하며 숫자를 버리지 않습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-add" locale="ko" />

## 관련 메서드

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
