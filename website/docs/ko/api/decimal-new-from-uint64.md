---
locale: ko
source_revision: a35340d9c55b8a08723d8ff0cfa4529bad65cad374d1045d7ce321047fc2a260
translation_provenance: en-US-korean-reviewed
api_id: Decimal::new_from_uint64
---
# Decimal::new_from_uint64

부호 없는 64비트 계수로 소수를 만듭니다. 음수가 아닌 정수 원본에 소수 자릿수를 별도로 지정할 때 사용합니다.

```rust
fn new_from_uint64 (value : u64 , prec : isize) -> Result < Self , DecimalError >
```

## 매개변수와 반환값

`value: u64` → 음수가 아닌 `bigint`, `prec: isize` → 정수 `number`. `value × 10^-prec`를 반환합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-from-uint64" locale="ko" />

## 사용 시 주의점과 제한

음수와 u64::MAX를 넘는 값은 Rust에 들어가기 전에 거부합니다. 음수 정밀도는 라이브러리 오류 `InvalidPrecision`입니다. JavaScript number는 u64 전체 범위를 안전하게 담을 수 없습니다.

브라우저는 [리소스 정책](../resource-policy)도 적용합니다. JavaScript 입력은 Rust 호출 전에 검사합니다.

## 관련 메서드

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
