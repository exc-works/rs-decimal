---
locale: ko
source_revision: ffab3b21dcd7c80e0222f2cd0457116e79f56aafc95fc2ed3c796e423b9661cc
translation_provenance: en-US-korean-reviewed
api_id: Decimal::new_from_big_rat
---
# Decimal::new_from_big_rat

유리수를 정확한 유한소수로 변환합니다.

```rust
fn new_from_big_rat (value : & BigRational) -> Result < Self , DecimalError >
```

## 매개변수, 반환값, 일반적인 사용법

value: &BigRational. JavaScript는 {numerator: bigint, denominator: bigint}를 받습니다. 분모는 0이 아니어야 하며 그렇지 않으면 JavaScript 매개변수 오류입니다. Rust BigRational은 부호와 공약수를 정규화합니다. 약분한 분모의 소인수가 2와 5뿐일 때만 유한소수이며 다른 인수는 InvalidFormat을 반환합니다. 결과는 초기화됩니다. 브라우저는 약분 후 검사하며 코어 변환 전에 출력 자릿수/계수 증가를 제한합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-new-from-big-rat" locale="ko" />

## 관련 메서드

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
