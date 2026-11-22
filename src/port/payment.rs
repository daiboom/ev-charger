use std::time::Duration;

use crate::domain::PaymentMethod;

#[derive(Debug, Clone, PartialEq)]
pub enum PaymentStatus {
    Idle,
    Pending {
        method: PaymentMethod,
    },
    Approved {
        method: PaymentMethod,
        transaction_id: String,
    },
    Declined {
        method: PaymentMethod,
        reason: String,
    },
}

pub trait PaymentPort {
    fn begin(&mut self, method: PaymentMethod);
    fn tick(&mut self, dt: Duration);
    fn status(&self) -> PaymentStatus;
    fn reset(&mut self);
}
