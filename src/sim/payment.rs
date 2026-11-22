use std::time::Duration;

use crate::domain::PaymentMethod;
use crate::port::{PaymentPort, PaymentStatus};

/// Approves every request after `delay` unless [`FakePayment::decline_next`] was set.
/// Transaction ids are local simulator ids. They are not processor references.
#[derive(Debug, Clone)]
pub struct FakePayment {
    delay: Duration,
    elapsed: Duration,
    status: PaymentStatus,
    next_id: u64,
    decline_next: bool,
}

impl FakePayment {
    pub fn new(delay: Duration) -> Self {
        Self {
            delay,
            elapsed: Duration::ZERO,
            status: PaymentStatus::Idle,
            next_id: 0,
            decline_next: false,
        }
    }

    pub fn immediate() -> Self {
        Self::new(Duration::ZERO)
    }

    /// The next `begin` declines instead of approving. The flag clears itself.
    pub fn decline_next(&mut self) {
        self.decline_next = true;
    }
}

impl PaymentPort for FakePayment {
    fn begin(&mut self, method: PaymentMethod) {
        self.elapsed = Duration::ZERO;
        if self.decline_next {
            self.decline_next = false;
            self.status = PaymentStatus::Declined {
                method,
                reason: "Simulated decline".to_string(),
            };
            return;
        }
        if self.delay.is_zero() {
            self.approve(method);
        } else {
            self.status = PaymentStatus::Pending { method };
        }
    }

    fn tick(&mut self, dt: Duration) {
        let PaymentStatus::Pending { method } = self.status else {
            return;
        };
        self.elapsed = self.elapsed.saturating_add(dt);
        if self.elapsed >= self.delay {
            self.approve(method);
        }
    }

    fn status(&self) -> PaymentStatus {
        self.status.clone()
    }

    fn reset(&mut self) {
        self.elapsed = Duration::ZERO;
        self.status = PaymentStatus::Idle;
    }
}

impl FakePayment {
    fn approve(&mut self, method: PaymentMethod) {
        self.next_id += 1;
        self.status = PaymentStatus::Approved {
            method,
            transaction_id: format!("SIM-{:04}", self.next_id),
        };
    }
}
