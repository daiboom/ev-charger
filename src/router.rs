use eframe::egui;
use std::path::PathBuf;
use std::time::Duration;
use crate::screen::{
    SplashScreen, ConnectScreen, StandbyScreen, FullChargeScreen,
    SelectAmountScreen, PaymentScreen, ChargingScreen, CompleteScreen,
    ChargeType, PaymentMethod
};

#[derive(Debug, Clone, PartialEq)]
pub enum AppState {
    Splash,
    Connect(ChargeType),
    Standby,
    FullCharge,
    SelectAmount(ChargeType),
    Payment(ChargeType, f32),
    Charging(ChargeType, f32, PaymentMethod),
    Complete(ChargeType, f32, PaymentMethod, f32, Duration),
}

// Navigation stack for maintaining screen history
pub type NavigationStack = Vec<AppState>;

pub struct Router {
    pub state: AppState,
    pub navigation_stack: NavigationStack,  // Screen history stack
    pub splash_screen: SplashScreen,
    pub connect_screen: ConnectScreen,
    pub standby_screen: StandbyScreen,
    pub full_charge_screen: FullChargeScreen,
    pub select_amount_screen: Option<SelectAmountScreen>,
    pub payment_screen: Option<PaymentScreen>,
    pub charging_screen: Option<ChargingScreen>,
    pub complete_screen: Option<CompleteScreen>,
}

impl Router {
    pub fn new() -> Self {
        // Background image path setup (optional)
        let splash_bg_path = std::path::PathBuf::from("assets/images/splash_bg.jpg");
        let connect_bg_path = std::path::PathBuf::from("assets/images/connect_bg.jpg");
        let standby_bg_path = std::path::PathBuf::from("assets/images/standby_bg.jpg");
        let full_charge_bg_path = std::path::PathBuf::from("assets/images/full_charge_bg.jpg");
        
        Self {
            state: AppState::Splash,
            navigation_stack: vec![AppState::Splash],  // Initialize with splash screen
            splash_screen: if splash_bg_path.exists() {
                SplashScreen::new().with_background_image(splash_bg_path)
            } else {
                SplashScreen::new()
            },
            connect_screen: if connect_bg_path.exists() {
                ConnectScreen::new().with_background_image(connect_bg_path)
            } else {
                ConnectScreen::new()
            },
            standby_screen: if standby_bg_path.exists() {
                StandbyScreen::new().with_background_image(standby_bg_path)
            } else {
                StandbyScreen::new()
            },
            full_charge_screen: if full_charge_bg_path.exists() {
                FullChargeScreen::new().with_background_image(full_charge_bg_path)
            } else {
                FullChargeScreen::new()
            },
            select_amount_screen: None,
            payment_screen: None,
            charging_screen: None,
            complete_screen: None,
        }
    }

    // Navigation stack management methods
    pub fn push_screen(&mut self, new_state: AppState) {
        // Don't push if it's the same as current state
        if self.state != new_state {
            self.navigation_stack.push(self.state.clone());
            self.state = new_state;
        }
    }

    pub fn pop_screen(&mut self) -> Option<AppState> {
        if self.navigation_stack.len() > 1 {
            let previous_state = self.navigation_stack.pop();
            if let Some(prev_state) = previous_state {
                self.state = prev_state;
                return Some(self.state.clone());
            }
        }
        None
    }

    pub fn can_go_back(&self) -> bool {
        self.navigation_stack.len() > 1
    }

    pub fn get_navigation_stack(&self) -> &NavigationStack {
        &self.navigation_stack
    }

    pub fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let current_state = self.state.clone();
        match current_state {
            AppState::Splash => {
                self.splash_screen.show(ctx);
                
                // Transition to connect screen when splash finishes
                if self.splash_screen.is_finished() {
                    self.push_screen(AppState::Connect(ChargeType::SpecificWatts(0.0)));
                }
            }
            AppState::Connect(_charge_type) => {
                self.connect_screen.show(ctx);
                
                // Transition to standby screen when connection is complete
                if self.connect_screen.is_connection_complete() {
                    self.push_screen(AppState::Standby);
                }
            }
            AppState::Standby => {
                self.standby_screen.show(ctx);
                
                // Transition to full charge screen when full charge button is clicked
                if self.standby_screen.is_full_charge_clicked() {
                    self.standby_screen.reset_full_charge_clicked();
                    self.push_screen(AppState::FullCharge);
                }
                 
                // Transition to connect screen when specific watts charge button is clicked
                if self.standby_screen.is_specific_watts_clicked() {
                    self.standby_screen.reset_specific_watts_clicked();
                    self.push_screen(AppState::Connect(ChargeType::SpecificWatts(0.0)));
                }
                 
                // Transition to connect screen when percent charge button is clicked
                if self.standby_screen.is_percent_clicked() {
                    self.standby_screen.reset_percent_clicked();
                    self.push_screen(AppState::Connect(ChargeType::Percent(0.0)));
                }
            }
            AppState::FullCharge => {
                self.full_charge_screen.show(ctx);
                
                // Return to previous screen when back button is clicked
                if self.full_charge_screen.is_back_clicked() {
                    self.full_charge_screen.reset_back_clicked();
                    self.pop_screen();
                }
                
                // Return to standby screen when charging is complete
                if self.full_charge_screen.is_charging_complete() {
                    self.state = AppState::Standby;
                }
            }
            AppState::SelectAmount(charge_type) => {
                let mut should_proceed = false;
                let mut should_go_back = false;
                let mut proceed_amount = 0.0;
                
                if let Some(ref mut screen) = self.select_amount_screen {
                    screen.show(ctx);
                    
                    if screen.is_proceed_clicked() {
                        screen.reset_proceed_clicked();
                        proceed_amount = screen.get_input_value();
                        should_proceed = true;
                    }
                    
                    if screen.is_back_clicked() {
                        screen.reset_back_clicked();
                        should_go_back = true;
                    }
                }
                
                if should_proceed {
                    self.push_screen(AppState::Payment(charge_type.clone(), proceed_amount));
                }
                if should_go_back {
                    self.pop_screen();
                }
            }
            AppState::Payment(charge_type, amount) => {
                let mut should_proceed = false;
                let mut should_go_back = false;
                let mut proceed_payment_method = None;
                
                if let Some(ref mut screen) = self.payment_screen {
                    screen.show(ctx);
                    
                    if screen.is_proceed_clicked() {
                        screen.reset_proceed_clicked();
                        proceed_payment_method = screen.get_selected_payment();
                        should_proceed = true;
                    }
                    
                    if screen.is_back_clicked() {
                        screen.reset_back_clicked();
                        should_go_back = true;
                    }
                }
                
                if should_proceed {
                    if let Some(payment_method) = proceed_payment_method {
                        self.push_screen(AppState::Charging(charge_type.clone(), amount, payment_method));
                    }
                }
                if should_go_back {
                    self.pop_screen();
                }
            }
            AppState::Charging(charge_type, amount, payment_method) => {
                let mut should_complete = false;
                let mut should_go_back = false;
                
                if let Some(ref mut screen) = self.charging_screen {
                    screen.show(ctx);
                    
                    if screen.is_charging_complete() {
                        should_complete = true;
                    }
                    
                    if screen.is_back_clicked() {
                        screen.reset_back_clicked();
                        should_go_back = true;
                    }
                }
                
                if should_complete {
                    // Transition to complete screen when charging is finished
                    let total_cost = match charge_type {
                        ChargeType::SpecificWatts(watts) => watts * 0.2,
                        ChargeType::Percent(percent) => percent * 0.1,
                    };
                    let charging_duration = Duration::from_secs(300); // 5 minute simulation
                    self.push_screen(AppState::Complete(
                        charge_type.clone(),
                        amount,
                        payment_method.clone(),
                        total_cost,
                        charging_duration,
                    ));
                }
                if should_go_back {
                    self.pop_screen();
                }
            }
            AppState::Complete(_, _, _, _, _) => {
                if let Some(ref mut screen) = self.complete_screen {
                    screen.show(ctx);
                    
                    if screen.is_return_home_clicked() {
                        screen.reset_return_home_clicked();
                        // Clear the navigation stack and go to standby
                        self.navigation_stack.clear();
                        self.navigation_stack.push(AppState::Standby);
                        self.state = AppState::Standby;
                    }
                }
            }
        }
    }

    // Helper methods for screen transitions
    pub fn go_to_standby(&mut self) {
        self.state = AppState::Standby;
    }

    pub fn go_to_full_charge(&mut self) {
        self.state = AppState::FullCharge;
    }

    pub fn go_to_splash(&mut self) {
        self.state = AppState::Splash;
    }

    pub fn go_to_select_amount(&mut self, charge_type: ChargeType) {
        let bg_path = std::path::PathBuf::from("assets/images/select_amount_bg.jpg");
        self.select_amount_screen = Some(
            if bg_path.exists() {
                SelectAmountScreen::new(charge_type.clone()).with_background_image(bg_path)
            } else {
                SelectAmountScreen::new(charge_type.clone())
            }
        );
        self.state = AppState::SelectAmount(charge_type);
    }

    pub fn go_to_payment(&mut self, charge_type: ChargeType, amount: f32) {
        let bg_path = std::path::PathBuf::from("assets/images/payment_bg.jpg");
        self.payment_screen = Some(
            if bg_path.exists() {
                PaymentScreen::new(charge_type.clone(), amount).with_background_image(bg_path)
            } else {
                PaymentScreen::new(charge_type.clone(), amount)
            }
        );
        self.state = AppState::Payment(charge_type, amount);
    }

    pub fn go_to_charging(&mut self, charge_type: ChargeType, amount: f32, payment_method: PaymentMethod) {
        let bg_path = std::path::PathBuf::from("assets/images/charging_bg.jpg");
        self.charging_screen = Some(
            if bg_path.exists() {
                ChargingScreen::new(charge_type.clone(), amount, payment_method.clone()).with_background_image(bg_path)
            } else {
                ChargingScreen::new(charge_type.clone(), amount, payment_method.clone())
            }
        );
        self.state = AppState::Charging(charge_type, amount, payment_method);
    }

    pub fn go_to_complete(&mut self, charge_type: ChargeType, amount: f32, payment_method: PaymentMethod, total_cost: f32, duration: Duration) {
        let bg_path = std::path::PathBuf::from("assets/images/complete_bg.jpg");
        self.complete_screen = Some(
            if bg_path.exists() {
                CompleteScreen::new(charge_type.clone(), amount, payment_method.clone(), total_cost, duration).with_background_image(bg_path)
            } else {
                CompleteScreen::new(charge_type.clone(), amount, payment_method.clone(), total_cost, duration)
            }
        );
        self.state = AppState::Complete(charge_type, amount, payment_method, total_cost, duration);
    }

    // Current state check methods
    pub fn is_splash(&self) -> bool {
        matches!(self.state, AppState::Splash)
    }

    pub fn is_standby(&self) -> bool {
        matches!(self.state, AppState::Standby)
    }

    pub fn is_full_charge(&self) -> bool {
        matches!(self.state, AppState::FullCharge)
    }
}
