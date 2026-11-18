---
locale: ko
source_revision: 78d4bcb73439eeefbcaa373b59f6572230d9fd809bde6e3821566e584e60e547
translation_provenance: en-US-korean-reviewed
api_id: Decimal::new_with_prec
---
# Decimal::new_with_prec

부호 있는 정수를 지정한 소수 자릿수의 계수로 해석합니다. 예를 들어 12와 정밀도 2는 0.12를 뜻합니다.

```rust
fn new_with_prec (value : i64 , prec : isize) -> Result < Self , DecimalError >
```

## 매개변수와 반환값

`value: i64` → `bigint`, `prec: isize` → 정수 `number`. `value × 10^-prec`를 반환합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-with-prec" locale="ko" />

## 사용 시 주의점과 제한

정밀도는 유효 숫자가 아니라 소수 자릿수입니다. 정수의 수치를 그대로 두고 0을 붙이는 것이 아닙니다. 12.00에는 `new_with_append_prec`를 사용하세요. 음수 정밀도는 `InvalidPrecision`입니다. 계수가 0이어도 요청한 소수 자릿수를 유지합니다.

브라우저는 [리소스 정책](../resource-policy)도 적용합니다. JavaScript 입력은 Rust 호출 전에 검사합니다.

## 관련 메서드

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
