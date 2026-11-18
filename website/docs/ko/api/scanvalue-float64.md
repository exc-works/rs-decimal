---
locale: ko
source_revision: e5103a64d713be93c5c51f691b143dc290327bd825e0acd2029d936b2165437e
translation_provenance: en-US-korean-reviewed
api_id: "ScanValue::Float64"
---
# ScanValue::Float64

SQL 입력 변형 Float64을 선택합니다.

```rust
Float64 (f64)
```

## 매개변수, 반환값, 일반적인 사용법

binary64를 담습니다. 유한 값은 코어의 최단 소수 변환을 사용하며 NaN과 무한대는 Unmarshal입니다. 이미 반올림된 JS number로 원래 숫자를 복원할 수 없습니다. 변형은 scan에 빌려 전달하며 한 브라우저 실행 안에서 재사용할 수 있습니다. 임의로 만든 값이나 다른 실행의 값은 거부합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-float64" locale="ko" />

## 브라우저 제한

SQL 텍스트/바이트 입력과 오류 메시지는 UTF-8 8192바이트까지입니다. SQL 텍스트는 바깥 따옴표 제거 후 [리소스 검사](../resource-policy)를 받으므로 따옴표로 지수 제한을 우회할 수 없습니다. 빌림 호출은 입력을 유지하며 소비되거나 해제된 핸들은 재사용할 수 없습니다. Rust 트레이트/가져오기 예제는 네이티브 테스트에서 실행하며 브라우저 메서드 지원을 뜻하지 않습니다.

## 관련 메서드

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
