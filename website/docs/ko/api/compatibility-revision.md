---
locale: ko
source_revision: 38cd98cd0eb4f9b158b56009f272819540665b187722c4eef35585af26929132
translation_provenance: en-US-korean-reviewed
api_id: COMPATIBILITY_REVISION
---
# COMPATIBILITY_REVISION

호환성 검사에 사용하는 업스트림 Go 리비전을 확인합니다.

```rust
const COMPATIBILITY_REVISION: & str = "a8791208e2d15483cf652cd2814b74b0c1a84187"
```

## 일반적인 사용법과 의미

이 불변 &str은 소스 리비전이며 현재 Rust/web 빌드 커밋이 아닙니다. 브라우저는 컴파일된 Rust에서 같은 상수를 읽습니다. 패키지 버전이나 배포 식별자로 해석하지 마세요.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-compatibility-revision" locale="ko" />

공유 예제는 네이티브 Rust와 브라우저에서 실행됩니다.

## 관련 문서

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
