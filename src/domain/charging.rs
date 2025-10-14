//! 충전 도메인 로직

use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChargingSession {
    pub id: String,
    pub status: ChargingStatus,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub battery_level: BatteryLevel,
    pub power: Power,
    pub energy: Energy,
    pub cost: Cost,
    pub target: ChargingTarget,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BatteryLevel(pub f32); // 0.0 ~ 100.0

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Power(pub f32); // kW

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Energy(pub f32); // kWh

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Cost(pub f32); // 원

impl ChargingSession {
    pub fn new(target: ChargingTarget) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            status: ChargingStatus::Idle,
            started_at: chrono::Utc::now(),
            battery_level: BatteryLevel(0.0),
            power: Power(0.0),
            energy: Energy(0.0),
            cost: Cost(0.0),
            target,
        }
    }
    
    pub fn start_charging(&mut self, power: Power) -> AppResult<()> {
        if !matches!(self.status, ChargingStatus::Idle) {
            return Err(AppError::ChargingError(
                "충전기가 사용 중입니다".to_string()
            ));
        }
        
        self.status = ChargingStatus::Charging;
        self.power = power;
        self.started_at = chrono::Utc::now();
        
        info!("충전 세션 시작: {}", self.id);
        Ok(())
    }
    
    pub fn update(&mut self, delta: Duration, config: &ChargingConfig) {
        if matches!(self.status, ChargingStatus::Charging) {
            let hours = delta.as_secs_f32() / 3600.0;
            let energy_delta = self.power.0 * hours;
            
            self.energy.0 += energy_delta;
            self.cost.0 = self.energy.0 * config.price_per_kwh;
            
            // 배터리 레벨 시뮬레이션 (실제로는 하드웨어에서 읽어옴)
            self.battery_level.0 = (self.battery_level.0 + energy_delta * 2.0).min(100.0);
            
            // 목표 달성 확인
            if self.is_target_reached() {
                self.status = ChargingStatus::Complete;
                info!("충전 완료: {}", self.id);
            }
        }
    }
    
    fn is_target_reached(&self) -> bool {
        match &self.target {
            ChargingTarget::Energy(target) => self.energy.0 >= *target,
            ChargingTarget::Cost(target) => self.cost.0 >= *target,
            ChargingTarget::Time(target) => {
                let elapsed = chrono::Utc::now() - self.started_at;
                elapsed >= chrono::Duration::from_std(*target).unwrap()
            }
            ChargingTarget::BatteryLevel(target) => self.battery_level.0 >= *target,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChargingStatus {
    Idle,
    Connecting,
    Charging,
    Paused,
    Complete,
    Error(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChargingTarget {
    Energy(f32),      // kWh
    Cost(f32),        // 원
    Time(Duration),   // 시간
    BatteryLevel(f32), // %
}
