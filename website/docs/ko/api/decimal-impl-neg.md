---
locale: ko
source_revision: 783c16e5cc4d9d8fc3c2d8c291b8f90a43746731baaf29ac25b9f64402f2b5c9
translation_provenance: en-US-korean-reviewed
api_id: "Decimal::impl Neg"
---
# Decimal::impl Neg

빌리는 Rust Neg 구문을 사용합니다.

```rust
impl Neg for & Decimal
```

## 일반적인 사용법과 주의사항

&Decimal에 구현되어 피연산자를 계속 사용할 수 있습니다. 단항 -는 소수 자릿수를 유지하고 기본 0을 포함한 결과를 초기화합니다. 트레이트는 같은 이름의 코어 메서드에 위임합니다. 브라우저 JS 연산자는 오버로드되지 않으므로 bridge의 add/sub/neg를 사용하세요.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-neg" locale="ko" />

## 실행

이 항목은 네이티브 Rust 트레이트 구문이며 같은 이름의 JS 연산자나 SQLx 객체를 제공하지 않습니다. 기본 네이티브 문서 테스트 실행기가 이 예제를 실행합니다.

## 관련 메서드

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
