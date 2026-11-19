---
locale: ko
source_revision: 2ea2866a9b7493c17d529f3372e5bbb238200663f6cd52e7c63d668cc43d0ffe
translation_provenance: en-US-korean-reviewed
api_id: Decimal::is_positive
---
# Decimal::is_positive

수치가 엄격히 양수인지 확인합니다.

```rust
fn is_positive (& self) -> bool
```

## 매개변수, 반환값, 사용 시 주의점

bool/boolean을 반환합니다. 미초기화 기본값을 포함한 0에는 false를 반환합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-is-positive" locale="ko" />

## 관련 메서드

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
