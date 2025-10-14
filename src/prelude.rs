//! 프로젝트 전역에서 자주 사용되는 타입들을 모은 prelude

// 외부 크레이트 재export
pub use eframe::egui;
pub use tokio;
pub use serde::{Deserialize, Serialize};
pub use tracing::{debug, error, info, warn};
pub use anyhow::{Context, Result};

// 내부 타입들 재export
pub use crate::error::{AppError, AppResult};
pub use crate::config::Config;

// 자주 사용되는 표준 라이브러리
pub use std::sync::Arc;
pub use std::time::{Duration, Instant};
