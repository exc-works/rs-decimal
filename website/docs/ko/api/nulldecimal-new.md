---
locale: ko
source_revision: 6d83b155d9084cec9f42deea0ac618a5403b16dd0003e0789794c0428f3a019a
translation_provenance: en-US-korean-reviewed
api_id: NullDecimal::new
---
# NullDecimal::new

소수를 이동하여 값이 있는 nullable로 만듭니다.

```rust
fn new (decimal : Decimal) -> Self
```

## 일반적인 사용법과 의미

decimal: Decimal을 소비합니다. decimal이 미초기화 0이어도 결과는 항상 valid=true입니다. 이동된 JavaScript 입력은 더 이상 사용할 수 없습니다. 원본이 계속 필요하면 복제하세요.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-nulldecimal-new" locale="ko" />

공유 예제는 네이티브 Rust와 브라우저에서 실행됩니다.

## 관련 문서

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
