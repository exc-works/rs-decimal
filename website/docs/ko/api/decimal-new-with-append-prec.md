---
locale: ko
source_revision: 845ec62ddce327ca5b797a7a47302c991bc3fbeb2f329134a32bdc0e5d88f53e
translation_provenance: en-US-korean-reviewed
api_id: Decimal::new_with_append_prec
---
# Decimal::new_with_append_prec

부호 있는 정수의 수치를 유지하며 저장할 소수 자릿수를 늘립니다. 정수 금액을 12.00처럼 나타낼 때 유용합니다.

```rust
fn new_with_append_prec (value : i64 , prec : isize) -> Result < Self , DecimalError >
```

## 매개변수와 반환값

`value: i64` → `bigint`, `prec: isize` → 정수 `number`. 저장된 계수는 `value × 10^prec`, 소수 자릿수는 `prec`가 됩니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-with-append-prec" locale="ko" />

## 사용 시 참고 사항

`string()`은 12를, `string_with_trailing_zeros()`는 12.00을 출력합니다. `new_with_prec`와 달리 수치가 0.12로 바뀌지 않습니다. 음수 정밀도는 `InvalidPrecision`입니다.

## 관련 메서드

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
