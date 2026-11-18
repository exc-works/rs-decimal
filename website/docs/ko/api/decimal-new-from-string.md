---
locale: ko
source_revision: 3ef0ab66b1050d81978ffe9504c298d164d62d052b64bfd1ad02ddce4a039f36
translation_provenance: en-US-korean-reviewed
api_id: Decimal::new_from_string
---
# Decimal::new_from_string

이진 부동소수점을 거치지 않고 정확한 소수 텍스트를 파싱합니다. 사용자가 입력한 소수 금액에 일반적으로 사용하는 생성자입니다.

```rust
fn new_from_string (text : & str) -> Result < Self , DecimalError >
```

## 매개변수와 반환값

`text: &str` → JavaScript `string`. Rust는 `Result<Decimal, DecimalError>`를 반환하며 bridge는 Decimal을 반환하거나 같은 라이브러리 오류 종류를 던집니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-from-string" locale="ko" />

## 사용 시 주의점과 제한

입력 앞뒤 공백을 제거하며 부호와 지수를 포함할 수 있습니다. 소수점 양쪽에 숫자가 필요합니다. 구분자는 UI 언어와 무관하게 항상 `.`입니다. 0이 아닌 수치의 후행 0은 유지하지만 `"0.00"`은 소수 자릿수 0인 초기화된 0으로 정규화합니다. 잘못된 문법은 `InvalidFormat`이며, 확장된 입력은 별도의 playground 리소스 제한에 걸릴 수 있습니다.

브라우저는 [리소스 정책](../resource-policy)도 적용합니다. JavaScript 입력은 Rust 호출 전에 검사합니다.

## 관련 메서드

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
