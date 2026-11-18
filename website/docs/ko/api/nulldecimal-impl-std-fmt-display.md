---
locale: ko
source_revision: 425ca68f655481c858e4668ea2e1df0ddc734454c90f72d356190ed7ac4eb22e
translation_provenance: en-US-korean-reviewed
api_id: "NullDecimal::impl std :: fmt :: Display"
---
# NullDecimal::impl std :: fmt :: Display

Rust Display로 nullable 값을 서식화합니다.

```rust
impl std :: fmt :: Display for NullDecimal
```

## 매개변수, 반환값, 일반적인 사용법

Display는 nullable string()에 위임하여 유효하면 내부의 정규 텍스트, 무효이면 null을 사용합니다. 포매터는 JSON 토큰이나 바이트 버퍼가 아니라 텍스트를 씁니다. format!은 Display::fmt를 호출하며 이 트레이트 문법은 네이티브에서 실행하고 브라우저 메서드가 아닙니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-nulldecimal-impl-std-fmt-display" locale="ko" />

## 브라우저 제한

디코더 입력은 8192바이트까지이며 매개변수 문자열도 Wasm에 복사하기 전에 길이를 검사합니다. Rust는 텍스트와 디코딩된 JSON 문자열에 [소수 리소스 정책](../resource-policy)을 다시 적용하므로 이스케이프로 지수/자릿수 제한을 우회할 수 없습니다. 잘못된 UTF-8과 JSON/소수 문법 진단은 코어가 처리합니다. 입력과 리소스 오류는 수신 객체를 부분적으로 갱신하지 않으며 이동된 래퍼는 읽거나 바꿀 수 없습니다.

## 관련 메서드

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
