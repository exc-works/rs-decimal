---
locale: ko
source_revision: 37b6ebebec69abdf2809e71eb7e3cad7353a792a069c7217103a51f2eea16c7f
translation_provenance: en-US-korean-reviewed
api_id: Decimal::new_from_decimal
---
# Decimal::new_from_decimal

저장된 표현과 초기화 상태를 포함하여 기존 Decimal을 복사합니다.

```rust
fn new_from_decimal (value : & Self) -> Self
```

## 매개변수와 반환값

`value: &Decimal`을 빌립니다. 반환된 Decimal은 독립적이며 입력도 계속 사용할 수 있습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-from-decimal" locale="ko" />

## 사용 시 주의점과 제한

`Decimal::default()`를 복사해도 미초기화 상태를 유지하며 초기화된 0으로 바꾸지 않습니다. JavaScript에서는 같은 실행에 속하는 사용 가능한 Decimal을 전달하세요. 이 생성자는 입력을 소비하지 않습니다.

브라우저는 [리소스 정책](../resource-policy)도 적용합니다. JavaScript 입력은 Rust 호출 전에 검사합니다.

## 관련 메서드

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
