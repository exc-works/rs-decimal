---
locale: ko
source_revision: 1534eba12b3e65df06a4bea76e8dd9c9cd12eeb6bcfbce92f69f83b923eef4c3
translation_provenance: en-US-korean-reviewed
api_id: "ScanValue::Int64"
---
# ScanValue::Int64

SQL 입력 변형 Int64을 선택합니다.

```rust
Int64 (i64)
```

## 매개변수, 반환값, 일반적인 사용법

i64를 정확히 담습니다. 브라우저는 부호 있는 64비트 범위의 bigint를 요구하고 JS number는 거부합니다. 변형은 scan에 빌려 전달하며 한 브라우저 실행 안에서 재사용할 수 있습니다. 임의로 만든 값이나 다른 실행의 값은 거부합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-int64" locale="ko" />

## 관련 메서드

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
