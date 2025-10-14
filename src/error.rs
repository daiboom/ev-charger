//! 통합 에러 타입 정의

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("충전 오류: {0}")]
    ChargingError(String),
    
    #[error("결제 오류: {0}")]
    PaymentError(String),
    
    #[error("하드웨어 오류: {0}")]
    HardwareError(String),
    
    #[error("설정 오류: {0}")]
    ConfigError(String),
    
    #[error("직렬화 오류: {0}")]
    SerdeError(#[from] serde_json::Error),
    
    #[error("IO 오류: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("TOML 파싱 오류: {0}")]
    TomlError(#[from] toml::de::Error),
}

// anyhow::Error를 AppError로 변환
impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::HardwareError(format!("내부 오류: {}", err))
    }
}

pub type AppResult<T> = Result<T, AppError>;
