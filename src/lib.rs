//! EV charger kiosk core.
//!
//! The binary owns the window. Charging decisions live here so they can be
//! tested without a display. The only charger adapter in this crate is a
//! simulator. Starting the process never restores a live session.

pub mod domain;
pub mod port;
pub mod sim;
