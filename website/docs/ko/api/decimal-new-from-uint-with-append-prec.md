---
locale: ko
source_revision: 276b5ccc2b5a962810bbd0b08cd40698f00e48d79a517b96783f1d6cae65ea86
translation_provenance: en-US-korean-reviewed
api_id: Decimal::new_from_uint_with_append_prec
---
# Decimal::new_from_uint_with_append_prec

부호 없는 정수의 수치를 유지하며 저장할 소수 자릿수를 추가합니다.

```rust
fn new_from_uint_with_append_prec (value : u64 , prec : isize) -> Result < Self , DecimalError >
```

## 매개변수와 반환값

`value: u64` → 음수가 아닌 `bigint`, `prec: isize` → 정수 `number`. 결과는 소수 자릿수 `prec`에서 정수 수치를 유지합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-from-uint-with-append-prec" locale="ko" />

## 사용 시 참고 사항

음수나 범위 밖의 값은 bridge 매개변수 오류입니다. 음수 정밀도는 `InvalidPrecision`이며 큰 정밀도는 별도의 playground 리소스 제한에 걸립니다. 후행 0을 유지하는 출력으로 추가된 자릿수를 확인하세요.

## 관련 메서드

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
