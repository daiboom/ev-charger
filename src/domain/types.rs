/// What the driver asked to buy. Energy and state of charge are targets.
/// Speed is a separate power cap, not a third meaning of the same number.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChargeTarget {
    /// Stop when the meter has delivered this much energy.
    Energy { kwh: f32 },
    /// Stop when the reported vehicle state of charge reaches this percent.
    Soc { percent: f32 },
    /// Stop when the vehicle reports the configured full state of charge.
    Full,
}

/// Power cap applied while the contactor is closed.
/// The kilowatt values come from [`crate::domain::SessionConfig`], not from this enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChargeSpeed {
    Fast,
    Normal,
    Slow,
}

impl ChargeSpeed {
    pub fn label(self) -> &'static str {
        match self {
            Self::Fast => "Fast",
            Self::Normal => "Normal",
            Self::Slow => "Slow",
        }
    }
}

/// Payment methods the kiosk can ask the fake authorizer to approve.
/// These are labels for the simulator. They do not talk to a terminal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaymentMethod {
    CreditCard,
    MobileApp,
    Rfid,
    Membership,
}

impl PaymentMethod {
    pub fn label(self) -> &'static str {
        match self {
            Self::CreditCard => "Credit card",
            Self::MobileApp => "Mobile app",
            Self::Rfid => "RFID card",
            Self::Membership => "Membership",
        }
    }

    pub fn all() -> [PaymentMethod; 4] {
        [
            Self::CreditCard,
            Self::MobileApp,
            Self::Rfid,
            Self::Membership,
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Selection {
    pub target: ChargeTarget,
    pub speed: ChargeSpeed,
}
