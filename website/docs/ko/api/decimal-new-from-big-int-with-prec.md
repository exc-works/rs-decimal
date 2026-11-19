---
locale: ko
source_revision: 89b834d2c11952f4f6bb0f4503b5e891bf052db7e48a72abe6c367e79b9b8c68
translation_provenance: en-US-korean-reviewed
api_id: Decimal::new_from_big_int_with_prec
---
# Decimal::new_from_big_int_with_prec

임의 크기의 계수를 복사하고 소수 자릿수를 지정합니다. 큰 정수 입력도 정확하게 유지합니다.

```rust
fn new_from_big_int_with_prec (value : & BigInt , prec : isize) -> Result < Self , DecimalError >
```

## 매개변수와 반환값

`value: &BigInt` → `bigint`, `prec: isize` → 정수 `number`. Rust 입력을 빌려 `value × 10^-prec`를 반환합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-from-big-int-with-prec" locale="ko" />

## 사용 시 참고 사항

계수의 숫자가 아니라 계수를 해석하는 방법을 바꿉니다. 음수 정밀도는 `InvalidPrecision`입니다.

## 관련 메서드

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
