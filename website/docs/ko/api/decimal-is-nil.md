---
locale: ko
source_revision: 7d0c030a450a67fa4b0e618dbc6cb501daeb5fbc64647ae39a02ca84be312b44
translation_provenance: en-US-korean-reviewed
api_id: Decimal::is_nil
---
# Decimal::is_nil

소수가 미초기화 기본값인지 확인합니다.

```rust
fn is_nil (& self) -> bool
```

## 매개변수, 반환값, 사용 시 주의점

bool/boolean을 반환합니다. 기본값은 nil이고 생성한 0은 초기화되어 있지만 수치로는 같습니다. NullDecimal의 유효성 플래그가 아닙니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-is-nil" locale="ko" />

## 관련 메서드

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
