use std::time::Duration;

use thiserror::Error;

/// Observed link between the connector and the vehicle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    Disconnected,
    /// Cable is detected. The vehicle has not released charging yet.
    Connected,
    /// Vehicle is ready. This is the only state that may accept a close command.
    Ready,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContactorCommand {
    Open,
    Closed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChargerFault {
    Internal,
    DisconnectedUnderLoad,
}

impl ChargerFault {
    pub fn message(self) -> &'static str {
        match self {
            Self::Internal => "Charger fault",
            Self::DisconnectedUnderLoad => "Connector unplugged during charging",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Meter {
    pub energy_kwh: f64,
    pub power_kw: f32,
    pub voltage_v: f32,
    pub current_a: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ChargerSnapshot {
    pub connector_id: u8,
    pub connection: ConnectionState,
    pub contactor: ContactorCommand,
    pub meter: Meter,
    pub vehicle_soc_percent: Option<f32>,
    pub battery_capacity_kwh: Option<f32>,
    pub fault: Option<ChargerFault>,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum PortError {
    #[error("contactor interlock rejected close: vehicle is not ready")]
    NotReady,
    #[error("contactor interlock rejected close: fault is active")]
    FaultActive,
}

/// Power-facing port. Implementations must reject a close that is not safe
/// on their own, even if the session also checks.
pub trait ChargerPort {
    fn snapshot(&self) -> ChargerSnapshot;
    fn set_contactor(&mut self, command: ContactorCommand) -> Result<(), PortError>;
    fn set_power_limit_kw(&mut self, kw: f32);
    fn tick(&mut self, dt: Duration);
}
