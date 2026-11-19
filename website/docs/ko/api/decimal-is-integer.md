---
locale: ko
source_revision: e73578c50b0181c9f243a7f97c0c8dbdcdf0b57587c7ac98a36dc6043a2ce666
translation_provenance: en-US-korean-reviewed
api_id: Decimal::is_integer
---
# Decimal::is_integer

수치에 소수 부분이 없는지, 즉 정수인지 확인합니다.

```rust
fn is_integer (& self) -> bool
```

## 매개변수, 반환값, 일반적인 사용법

매개변수 없음. bool/boolean을 반환합니다. 소수 자릿수가 0인지가 아니라 수치를 검사하므로 12.00도 정수입니다. 기본 0도 정수이며 소수 부분이 없습니다. 이 조회는 입력을 초기화하거나 변경하지 않습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-is-integer" locale="ko" />

## 관련 메서드

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
