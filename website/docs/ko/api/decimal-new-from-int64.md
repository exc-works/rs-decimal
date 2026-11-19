---
locale: ko
source_revision: fae9d1b20487be5ea60841b603ab060b40d5b639bd9efc686e9fd8f0f555bd79
translation_provenance: en-US-korean-reviewed
api_id: Decimal::new_from_int64
---
# Decimal::new_from_int64

부호 있는 64비트 계수와 소수 자릿수로 소수를 만듭니다. 수치와 상태의 동작은 `new_with_prec`와 같습니다.

```rust
fn new_from_int64 (value : i64 , prec : isize) -> Result < Self , DecimalError >
```

## 매개변수와 반환값

`value: i64` → `bigint`, `prec: isize` → 정수 `number`. `value × 10^-prec`를 반환합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-from-int64" locale="ko" />

## 사용 시 참고 사항

값은 계수이며 이미 스케일이 적용된 금액이 아닙니다. 음수 정밀도는 `InvalidPrecision`이고 i64 범위 밖의 값은 JavaScript 매개변수 오류입니다.

## 관련 메서드

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
