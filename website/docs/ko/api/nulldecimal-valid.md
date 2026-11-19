---
locale: ko
source_revision: 2a36a5c46f57052b68644728359c44f15465f6bfca2c60d1540e40c603cc8c5b
translation_provenance: en-US-korean-reviewed
api_id: NullDecimal.valid
---
# NullDecimal.valid

저장된 소수를 바꾸지 않고 값의 존재 여부를 읽거나 갱신합니다.

```rust
valid: bool
```

## 일반적인 사용법과 의미

이 공개 Rust bool 필드는 boolean getter/setter에 대응합니다. false로 설정해도 소수를 지우지 않으며 true로 설정해도 초기화하지 않습니다. JavaScript는 변경 전에 boolean이 아닌 값을 거부합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-nulldecimal-valid" locale="ko" />

공유 예제는 네이티브 Rust와 브라우저에서 실행됩니다.

## 관련 문서

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
