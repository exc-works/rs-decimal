---
locale: ko
source_revision: 7a9119eb74e99c5e35f09e87df2556b438ba2f0ad510d5e668cdcc13dc40e550
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

## 브라우저 제한

SQL 텍스트/바이트 입력과 오류 메시지는 UTF-8 8192바이트까지입니다. SQL 텍스트는 바깥 따옴표 제거 후 [리소스 검사](../resource-policy)를 받으므로 따옴표로 지수 제한을 우회할 수 없습니다. 빌림 호출은 입력을 유지하며 소비되거나 해제된 핸들은 재사용할 수 없습니다. Rust 트레이트/가져오기 예제는 네이티브 테스트에서 실행하며 브라우저 메서드 지원을 뜻하지 않습니다.

## 관련 메서드

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
