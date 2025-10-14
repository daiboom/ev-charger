//! EV 충전기 키오스크 애플리케이션

pub mod prelude;
pub mod config;
pub mod error;
pub mod app;
pub mod types;
pub mod ui;
pub mod domain;
pub mod services;
pub mod infrastructure;
pub mod utils;

// 메인 exports
pub use app::EvChargerApp;
pub use config::Config;
pub use error::{AppError, AppResult};
