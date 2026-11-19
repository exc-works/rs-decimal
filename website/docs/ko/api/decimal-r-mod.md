---
locale: ko
source_revision: 68a6c601d819a031d04a43cce835c2569811cfe3ffa0049911e2723c800f6029
translation_provenance: en-US-korean-reviewed
api_id: Decimal::r#mod
---
# Decimal::r#mod

0 방향으로 버리는 나눗셈의 부호 있는 나머지를 반환합니다.

```rust
fn r#mod (& self , other : & Self) -> Result < Self , DecimalError >
```

## 매개변수, 반환값, 일반적인 사용법

other: &Decimal. Rust는 Result<Decimal, DecimalError>를 반환합니다. 나머지는 피제수의 부호와 두 피연산자 중 큰 소수 자릿수를 가지며 음수가 아닌 유클리드 나머지가 아닙니다. 제수가 0이면 DivideByZero입니다. 입력은 빌려 사용합니다. mod는 키워드이므로 Rust에서는 r#mod, JS에서는 .mod(other)를 사용합니다. 두 이름 모두 같은 코어 동작을 호출합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-r-mod" locale="ko" />

## 관련 메서드

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
