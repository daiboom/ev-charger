#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use ev_charger::prelude::*;
use ev_charger::{EvChargerApp, Config};
use ev_charger::app::CommunicationEvent;
use ev_charger::infrastructure::communication::CommunicationManager;
use tokio::sync::mpsc;
use tracing::{info, error, warn};

#[tokio::main]
async fn main() -> AppResult<()> {
    // 로깅 시스템 초기화
    init_logging()?;
    
    info!("EV 충전기 키오스크 시작");
    
    // 설정 로드
    let config = load_config().await?;
    info!("설정 로드 완료");
    
    // 시스템 초기화 및 자가진단
    perform_system_check(&config).await?;
    
    // 통신 시스템 초기화 (백그라운드)
    let (communication_handle, comm_events, comm_sender) = init_communication_system(config.clone()).await?;

    // Ctrl+C 핸들러 설정
    let shutdown_sender = comm_sender.clone();
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.expect("failed to install CTRL+C signal handler");
        warn!("Ctrl-C 수신, 애플리케이션 종료 신호 전송...");
        if let Err(e) = shutdown_sender.send(CommunicationEvent::Shutdown) {
            error!("UI 종료 신호 전송 실패: {}", e);
        }
    });
    
    // UI 애플리케이션 실행 (메인 스레드)
    let ui_result = run_ui_application(config.clone(), comm_events).await;
    
    // 정리 작업
    info!("애플리케이션 종료 중...");
    communication_handle.abort();
    
    match ui_result {
        Ok(_) => {
            info!("애플리케이션이 정상적으로 종료됨");
            Ok(())
        }
        Err(e) => {
            error!("애플리케이션 오류로 종료: {}", e);
            Err(e)
        }
    }
}

fn init_logging() -> AppResult<()> {
    use tracing_subscriber::EnvFilter;
    
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("ev_charger=info"));
    
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .init();
    
    Ok(())
}

async fn load_config() -> AppResult<Config> {
    let config_paths = ["./config.toml", "/etc/ev-charger/config.toml"];
    
    for path in &config_paths {
        if std::path::Path::new(path).exists() {
            info!("설정 파일 로드: {}", path);
            return Config::load_from_file(path).await
                .map_err(|e| AppError::ConfigError(format!("설정 파일 로드 실패 {}: {}", path, e)));
        }
    }
    
    warn!("설정 파일을 찾을 수 없어 기본 설정 사용");
    Ok(Config::default())
}

async fn perform_system_check(_config: &Config) -> AppResult<()> {
    info!("시스템 자가진단 시작");
    
    // 기본 파일 시스템 확인
    check_file_system_permissions().await?;
    
    info!("시스템 자가진단 완료");
    Ok(())
}

async fn check_file_system_permissions() -> AppResult<()> {
    info!("파일 시스템 권한 확인 중...");
    
    let log_dir = std::path::Path::new("./logs");
    if !log_dir.exists() {
        tokio::fs::create_dir_all(log_dir).await
            .map_err(|e| AppError::IoError(e))?; // .context() 대신 직접 변환
    }
    
    Ok(())
}

async fn init_communication_system(
    config: Config,
) -> AppResult<(
    tokio::task::JoinHandle<()>, 
    mpsc::UnboundedReceiver<CommunicationEvent>,
    mpsc::UnboundedSender<CommunicationEvent>
)> {
    info!("통신 시스템 초기화 중...");
    
    let (event_tx, event_rx) = mpsc::unbounded_channel();
    let comm_manager = CommunicationManager::new(&config).await?;
    
    let handle = tokio::spawn(async move {
        info!("통신 시스템 백그라운드 태스크 시작");
        
        let mut heartbeat_interval = tokio::time::interval(Duration::from_secs(30));
        
        loop {
            tokio::select! {
                _ = heartbeat_interval.tick() => {
                    if let Err(e) = comm_manager.send_heartbeat().await {
                        error!("하트비트 전송 실패: {}", e);
                    }
                }
            }
        }
    });
    
    Ok((handle, event_rx, event_tx))
}

async fn run_ui_application(
    config: Config,
    comm_events: mpsc::UnboundedReceiver<CommunicationEvent>,
) -> AppResult<()> {
    info!("UI 애플리케이션 시작");
    
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 800.0]) // Set to 400x800
            .with_min_inner_size([400.0, 800.0]) // Prevent resizing
            .with_resizable(false) // Disable resizing
            .with_decorations(!config.ui.fullscreen)
            .with_always_on_top()
            .with_fullscreen(config.ui.fullscreen),
        
        renderer: eframe::Renderer::Glow,
        run_and_return: false,
        
        ..Default::default()
    };
    
    let app_result = eframe::run_native(
        "EV 충전기 키오스크",
        native_options,
        Box::new(move |cc| {
            ev_charger::ui::setup_fonts(&cc.egui_ctx, &config.ui);
            
            Ok(Box::new(EvChargerApp::new(config, comm_events)))
        }),
    );
    
    match app_result {
        Ok(_) => {
            info!("UI 애플리케이션이 정상적으로 종료됨");
            Ok(())
        }
        Err(e) => {
            error!("UI 애플리케이션 실행 실패: {}", e);
            Err(AppError::ChargingError(format!("UI 실행 실패: {}", e)))
        }
    }
}
