---
locale: ko
source_revision: 3a7d890770468c269032ca95ee3365a1f5fdeb70051590bf674c733d1e1658d6
translation_provenance: en-US-korean-reviewed
api_id: Decimal::is_zero
---
# Decimal::is_zero

값에 따라 달라지는 연산 전에 수치가 0인지 확인합니다.

```rust
fn is_zero (& self) -> bool
```

## 매개변수, 반환값, 사용 시 주의점

bool/boolean을 반환하며 미초기화 기본값에도 true를 반환합니다. 초기화 상태가 중요하면 is_nil()을 사용하세요.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-is-zero" locale="ko" />

## 관련 메서드

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
