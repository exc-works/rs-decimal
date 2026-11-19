---
locale: ko
source_revision: eb4bab6a9fdfe771d31eefc8731c88dad81f2f8a3273adea4040fea11f4141ab
translation_provenance: en-US-korean-reviewed
api_id: Decimal::sign
---
# Decimal::sign

부호를 -1, 0, 1로 읽습니다.

```rust
fn sign (& self) -> i32
```

## 매개변수, 반환값, 사용 시 주의점

i32/number를 반환합니다. 초기화된 0과 미초기화 0의 부호는 모두 0이며 음의 0을 별도로 구분하지 않습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-sign" locale="ko" />

## 관련 메서드

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
