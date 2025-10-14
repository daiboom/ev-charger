//! 애플리케이션 설정 관리

use serde::{Deserialize, Serialize};
use std::path::Path;
use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub ui: UiConfig,
    pub charging: ChargingConfig,
    pub payment: PaymentConfig,
    pub hardware: HardwareConfig,
    pub ocpp: Option<OcppConfig>,
    pub modbus: Option<ModbusConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    pub theme: String,
    pub font_size_multiplier: f32,
    pub fullscreen: bool,
    pub always_on_top: bool,
    pub screen_width: f32,
    pub screen_height: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChargingConfig {
    pub max_power: f32,          // kW
    pub price_per_kwh: f32,      // 원/kWh
    pub session_timeout: u64,    // 초
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentConfig {
    pub supported_methods: Vec<String>,
    pub api_endpoint: String,
    pub timeout: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareConfig {
    pub enable_can: bool,
    pub enable_gpio: bool,
    pub can_interface: String,
    pub serial_ports: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcppConfig {
    pub server_url: String,
    pub charger_id: String,
    pub heartbeat_interval: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModbusConfig {
    pub address: String,
    pub port: u16,
    pub slave_id: u8,
}

impl Config {
    pub async fn load_from_file<P: AsRef<Path>>(path: P) -> AppResult<Self> {
        let content = tokio::fs::read_to_string(path).await?;
        let config = toml::from_str(&content)?;
        Ok(config)
    }
    
    pub fn default() -> Self {
        Self {
            ui: UiConfig {
                theme: "dark".to_string(),
                font_size_multiplier: 1.0,
                fullscreen: true,
                always_on_top: true,
                screen_width: 1024.0,
                screen_height: 768.0,
            },
            charging: ChargingConfig {
                max_power: 150.0,
                price_per_kwh: 350.0,
                session_timeout: 1800,
            },
            payment: PaymentConfig {
                supported_methods: vec!["card".to_string(), "mobile".to_string()],
                api_endpoint: "https://api.evcharger.com".to_string(),
                timeout: 30,
            },
            hardware: HardwareConfig {
                enable_can: false,
                enable_gpio: false,
                can_interface: "can0".to_string(),
                serial_ports: vec!["/dev/ttyUSB0".to_string()],
            },
            ocpp: Some(OcppConfig {
                server_url: "ws://localhost:8080".to_string(),
                charger_id: "CHARGER001".to_string(),
                heartbeat_interval: 30,
            }),
            modbus: None,
        }
    }
}
