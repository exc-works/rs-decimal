---
locale: ko
source_revision: 56094af9f622985dec933077af65cd0cc673023dc5a3a4be3553f2af47b621d4
translation_provenance: en-US-korean-reviewed
api_id: Decimal
---
# Decimal

정확한 계수와 음수가 아닌 소수 자릿수를 저장합니다.

```rust
pub struct Decimal
```

## 일반적인 사용법과 의미

수치는 계수 × 10^-scale입니다. scale은 소수 자릿수이며 유지된 후행 0도 포함합니다. 기본값은 미초기화 수치 0입니다. 생성자는 초기화된 값을 만들고, 빌림 메서드는 입력을 유지하며, 소비 메서드는 입력의 브라우저 래퍼를 무효화합니다. JavaScript에서는 이름이 있는 생성자를 사용하세요. new Decimal()은 공개 생성자가 아닙니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal" locale="ko" />

공유 예제는 네이티브 Rust와 브라우저에서 실행됩니다.

## 관련 문서

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
