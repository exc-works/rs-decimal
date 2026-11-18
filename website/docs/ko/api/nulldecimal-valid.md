---
locale: ko
source_revision: d57c1e8ddb07024ed3c5c002f095b2cee086c1a0719df70a0b4ba8761f29418d
translation_provenance: en-US-korean-reviewed
api_id: NullDecimal.valid
---
# NullDecimal.valid

저장된 소수를 바꾸지 않고 값의 존재 여부를 읽거나 갱신합니다.

```rust
valid: bool
```

## 일반적인 사용법과 의미

이 공개 Rust bool 필드는 boolean getter/setter에 대응합니다. false로 설정해도 소수를 지우지 않으며 true로 설정해도 초기화하지 않습니다. bridge는 변경 전에 boolean이 아닌 값을 거부합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-nulldecimal-valid" locale="ko" />

공유 예제는 네이티브 Rust와 브라우저에서 실행됩니다.

## 관련 문서

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
