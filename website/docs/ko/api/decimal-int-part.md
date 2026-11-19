---
locale: ko
source_revision: b7ff1544694c51babb7af7cb99735ecc6687d86e9cd9a108af321bed93252beb
translation_provenance: en-US-korean-reviewed
api_id: Decimal::int_part
---
# Decimal::int_part

0 방향으로 버려 정수 부분을 추출합니다.

```rust
fn int_part (& self) -> BigInt
```

## 매개변수, 반환값, 일반적인 사용법

매개변수 없음. Rust는 BigInt, bridge는 bigint를 반환하며 부동소수점 변환을 하지 않습니다. -12.75의 결과는 -12이며 floor(-12.75) = -13이 아닙니다. 기본 0은 0을 반환합니다. 예제는 JSON에서 임의 정밀도를 유지하도록 정수를 문자열로 직렬화합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-int-part" locale="ko" />

## 관련 메서드

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
