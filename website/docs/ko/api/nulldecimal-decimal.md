---
locale: ko
source_revision: 74688d1ee73b78a25c1792ba73a6a2e013084b0279376cef739a1b5c8cbed64f
translation_provenance: en-US-korean-reviewed
api_id: NullDecimal.decimal
---
# NullDecimal.decimal

유효성과 별개로 저장된 소수를 확인하거나 교체합니다.

```rust
decimal: Decimal
```

## 일반적인 사용법과 의미

Rust는 수정 가능한 Decimal 필드를 공개합니다. 브라우저는 get_decimal_clone()으로 독립적인 복사본을 얻고 set_decimal(value)로 교체값을 소비합니다. 두 어댑터 모두 valid를 바꾸지 않습니다. 이 명시적 어댑터는 JavaScript에서 Rust의 실시간 참조를 제공한다는 오해를 피합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-nulldecimal-decimal" locale="ko" />

공유 예제는 네이티브 Rust와 브라우저에서 실행됩니다.

## 관련 문서

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
