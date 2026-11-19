---
locale: ko
source_revision: da8cebe24a220cbee0329a50db3c796d6390c270eb7d574cffaac17c4ddc1264
translation_provenance: en-US-korean-reviewed
api_id: "NullDecimal::impl std :: fmt :: Display::fmt"
---
# NullDecimal::impl std :: fmt :: Display::fmt

Rust Display로 nullable 값을 서식화합니다.

```rust
fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result
```

## 매개변수, 반환값, 일반적인 사용법

Display는 nullable string()에 위임하여 유효하면 내부의 정규 텍스트, 무효이면 null을 사용합니다. 포매터는 JSON 토큰이나 바이트 버퍼가 아니라 텍스트를 씁니다. format!은 Display::fmt를 호출하며 이 트레이트 문법은 네이티브에서 실행하고 브라우저 메서드가 아닙니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-nulldecimal-impl-std-fmt-display-fmt" locale="ko" />

## 관련 메서드

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
