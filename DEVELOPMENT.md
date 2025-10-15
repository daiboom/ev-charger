EV Charger Kiosk – 개발 가이드

## 사전 준비물
- Rust (rustup로 설치)
- cargo

## 실행 (개발 모드)
```
cargo run
```

자동 재시작(권장):
```
cargo install cargo-watch
cargo watch -x run
```

## 프로젝트 구조
```
src/
  main.rs                    # 앱 진입점, 앱 상태
  layout/
    mod.rs                  # 레이아웃 모듈
    top_bar.rs              # 상단 바 UI (시계, 창 컨트롤)
    bottom_bar.rs           # 하단 바 UI (상태, 설정)
    stepper.rs              # 단계 표시 컴포넌트 (Select → Payment → Charging → Complete)
  screen/
    mod.rs                  # 화면 모듈
    splash_screen.rs        # 스플래시 (배경 이미지 선택적)
    standby_screen.rs       # 대기/진입 화면 (액션 버튼)
assets/
  images/
    splash_bg.jpg           # 스플래시 배경 (선택)
    standby_bg.jpg          # 스탠바이 배경 (선택)
```

## 현재 화면
- Splash: 간단한 인트로, 자동으로 Standby로 전환
- Standby: 반응형 레이아웃, 상단에 단계 표시, 액션 버튼:
  - Charge specific watts
  - Charge by %
  - Full charge

## 반응형 규칙
- 스케일 팩터 = min(width/800, height/600), 범위 [0.6, 2.0]로 클램프
- 버튼/폰트/여백은 모두 이 스케일에 비례
- 좁은 화면에서는 버튼을 세로 스택, 충분히 넓으면 가로 정렬

## 배경 이미지
- 선택적으로 `assets/images/`에 아래 이름으로 배치:
  - `splash_bg.jpg`
  - `standby_bg.jpg`
- 파일이 있으면 런타임에 로드, 없으면 기본 배경색 사용

## 창 컨트롤
`main.rs` 설정:
- `.with_decorations(true)` (OS 닫기/최소화/최대화)
- `.with_resizable(true)`
- 전체화면/최소화 등 커스텀 컨트롤은 `layout/top_bar.rs`에 구현

## 단계(stepper) 컴포넌트
어디서든 아래처럼 사용:
```
use crate::layout::stepper::stepper;
stepper(ui, &["Select amount", "Payment method", "Charging", "Complete"], current_index, scale);
```

## 새 화면 추가 방법
1) 파일 생성: `src/screen/<your_screen>.rs`에 `pub struct`와 `pub fn show(&mut self, ctx: &egui::Context)` 구현
2) `src/screen/mod.rs`에 export 추가
3) `main.rs`(또는 코디네이터)에서 상태 필드/라우팅 추가 (`eframe::App::update`)
4) 필요 시 stepper의 현재 단계 인덱스 갱신

## Standby 액션을 플로우로 연결
현재는 로그만 남김. 실제 플로우를 구동하려면:
1) `StandbyScreen::show`가 enum(예: `Option<StandbyAction>`)을 반환하도록 변경
2) `main.rs`에서 해당 액션을 매칭해 Select/Payment/Charging 상태로 전환
3) stepper의 현재 인덱스로 활성 단계 반영

## 결제 수단
계획: 현금, 카드, 카카오페이 등
- `PaymentScreen`에서 결제 수단 버튼 구성
- 카드/카카오페이는 ProgressBar로 진행 상태 시뮬레이션
- 결제 완료 후 결제 금액/거래 ID를 앱 상태로 전달

## 충전 상태/완료
- Charging 화면: 실시간 전력/전류, 예상 시간/금액, 중지 버튼
- Complete 화면: 실제 충전량(kWh), 최종 결제 금액, 마무리 액션

## 로깅
개발 중에는 `println!` 사용. 배포용은 필요 시 `tracing` 도입 권장

## 스타일 가이드
- 바 영역은 `egui::TopBottomPanel`, 콘텐츠는 `CentralPanel` 권장
- 크기/폰트는 항상 `scale` 곱으로 일관성 유지

## 진행 예정 작업(TODO)
- Standby 버튼 액션을 다음 화면으로 라우팅
- Payment/Charging/Completion 화면 구현
- 간단한 설정(언어, 테마) 보존

## 트러블슈팅
- 이미지가 안 보이면 `assets/images/*_bg.jpg` 존재 확인 (없으면 기본 배경 사용)
- 창 버튼이 안 보이면 `.with_decorations(true)` 설정 및 전체화면 해제 확인
