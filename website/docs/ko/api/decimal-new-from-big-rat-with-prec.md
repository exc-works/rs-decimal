---
locale: ko
source_revision: 75750ddb09ce29d94b77a840b238d39bf569cfb662c01d997d14bb79d57564c3
translation_provenance: en-US-korean-reviewed
api_id: Decimal::new_from_big_rat_with_prec
---
# Decimal::new_from_big_rat_with_prec

유리수를 명시한 소수 자릿수로 반올림합니다.

```rust
fn new_from_big_rat_with_prec (value : & BigRational , prec : isize , mode : RoundingMode ,) -> Result < Self , DecimalError >
```

## 매개변수, 반환값, 일반적인 사용법

value: &BigRational, prec: 음수가 아닌 isize, mode: RoundingMode. bridge는 정확한 분자/분모 bigint 객체를 사용합니다. 정확한 변환과 달리 1/3 같은 무한소수도 허용합니다. 음수 정밀도는 InvalidPrecision, Unnecessary에서 반올림이 필요하면 RoundUnnecessary를 반환합니다. 0인 분모는 Rust 호출 전 bridge 입력 오류이며 0인 분자는 소수 자릿수 prec의 초기화된 0을 반환합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-new-from-big-rat-with-prec" locale="ko" />

## 브라우저 제한

정수 전송은 손실이 있는 JS number를 거치지 않습니다. 변환 튜플은 Rust의 값/플래그 순서를 유지하며 false는 정확한 결과를 보장하지 않습니다. 빌린 입력은 바뀌지 않고 생성자는 결과를 초기화합니다. [리소스 정책](../resource-policy)은 라이브러리 오류와 별개로 정수 입력, 출력 자릿수, 유리수 계수 증가를 제한합니다. Rust 가져오기 문법은 네이티브에서 검사한다고 명시합니다.

## 관련 메서드

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
