---
locale: ko
source_revision: aeb152b6bffb5f406cc2fbe512dd2449186121fa1cd03efbf5d74f0d3891aed1
translation_provenance: en-US-korean-reviewed
api_id: "ErrorKind::NoConvergence"
---
# ErrorKind::NoConvergence

NoConvergence 오류를 식별합니다.

```rust
NoConvergence
```

## 매개변수, 반환값, 일반적인 사용법

제한된 알고리즘이 반올림 결과를 증명하지 못했습니다. 수학적 결과가 없다는 뜻은 아닙니다. ErrorKind는 메시지 문자열이 아니라 안정적인 분류입니다. 브라우저 열거값은 Rust 변형 이름입니다. 생성한 오류는 matches()를 사용하고 잡은 bridge 예외는 kinds 배열을 사용합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-noconvergence" locale="ko" />

## 관련 메서드

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
