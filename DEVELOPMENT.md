# 개발 노트

바이너리 `ev-charger`가 창을 열고, 라이브러리가 세션과 시뮬레이터를 가집니다.

```text
src/main.rs                  창, 전체화면 인자, tracing
src/router.rs                화면과 세션 연결
src/domain/session.rs        상태기계와 컨택터 인터록
src/port/                    충전기, 결제 포트
src/sim/                     시뮬레이터와 가짜 승인
src/screen/                  egui 화면
src/layout/                  공통 창 요소
```

`cargo test`는 라이브러리의 세션 테스트와 금액 파싱 테스트를 실행합니다.

충전 중 뒤로 가기는 없습니다. 충전 화면의 Stop이 세션 중지를 호출하고 컨택터를 엽니다. 연결과 결제 화면의 Back은 아직 출력을 시작하지 않은 세션을 대기로 되돌립니다.

하드웨어 어댑터는 포트 뒤에 추가할 수 있습니다. 신호 명세가 생기기 전에는 시뮬레이터만 둡니다.
