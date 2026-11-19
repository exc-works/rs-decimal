---
locale: ko
source_revision: 3d99b7dec25de936ed85161e3eb8cf4565f56b88649e6c879a6468d051308404
translation_provenance: en-US-korean-reviewed
api_id: Decimal::new_from_big_rat_with_prec
---
# Decimal::new_from_big_rat_with_prec

유리수를 명시한 소수 자릿수로 반올림합니다.

```rust
fn new_from_big_rat_with_prec (value : & BigRational , prec : isize , mode : RoundingMode ,) -> Result < Self , DecimalError >
```

## 매개변수, 반환값, 일반적인 사용법

value: &BigRational, prec: 음수가 아닌 isize, mode: RoundingMode. JavaScript는 정확한 분자/분모 bigint 객체를 사용합니다. 정확한 변환과 달리 1/3 같은 무한소수도 허용합니다. 음수 정밀도는 InvalidPrecision, Unnecessary에서 반올림이 필요하면 RoundUnnecessary를 반환합니다. 0인 분모는 Rust 호출 전 JavaScript 입력 오류이며 0인 분자는 소수 자릿수 prec의 초기화된 0을 반환합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-new-from-big-rat-with-prec" locale="ko" />

## 관련 메서드

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
