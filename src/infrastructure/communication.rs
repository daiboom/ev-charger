//! 통신 매니저

use crate::prelude::*;


pub struct CommunicationManager {
    config: Config,
}



#[derive(Debug, Clone)]
pub struct PowerMeterData {
    pub power: f32,
}

impl PowerMeterData {
    pub fn simulated() -> Self {
        Self { power: 50.0 }
    }
}

impl CommunicationManager {
    pub async fn new(config: &Config) -> AppResult<Self> {
        Ok(Self {
            config: config.clone(),
        })
    }
    
    pub async fn send_heartbeat(&self) -> AppResult<()> {
        info!("하트비트 전송");
        Ok(())
    }
    
    pub async fn read_power_meter(&self) -> AppResult<PowerMeterData> {
        Ok(PowerMeterData::simulated())
    }
    
    pub async fn receive_can_message(&self) -> AppResult<Vec<u8>> {
        // 시뮬레이션 데이터
        Ok(vec![0x01, 0x02, 0x03, 0x04])
    }
}
